use crate::commands::bindings::MatchBinding;
use crate::common::get_params_from_path;
use crate::config::{Endpoint, HtrsConfig, Service};
use crate::htrs_binding_error::HtrsBindingError;
use crate::outcomes::HtrsAction::MakeRequest;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgAction, ArgMatches, Command};
use reqwest::{Method, Url};
use std::collections::HashMap;

pub struct CallServiceEndpointCommand {
    pub service_name: String,
    pub environment_name: Option<String>,
    pub path: String,
    pub query_parameters: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub show_body: bool,
    pub preset: Option<String>,
}

impl CallServiceEndpointCommand {
    pub fn get_command(config: &HtrsConfig) -> Command {
        let mut command = Command::new("call")
            .about("Call a service endpoint")
            .arg(
                Arg::new("environment")
                    .value_name("environment name")
                    .required(false)
                    .help("Environment to target, will use default environment if none specified")
                    .long("environment")
                    .short('e')
            )
            .arg_required_else_help(true);

        for service in &config.services {
            command = command.subcommand(get_command_for_service(service));
        }

        command
    }

    pub fn bind_from_matches(config: &HtrsConfig, args: &ArgMatches) -> Result<CallServiceEndpointCommand, HtrsBindingError> {
        let Some((service_name, service_matches)) = args.subcommand() else {
            panic!("Bad service subcommand for CallServiceEndpointCommand");
        };
        let Some(service) = config.get_service(service_name) else {
            panic!("Bad service name");
        };

        let Some((endpoint_name, endpoint_matches)) = service_matches.subcommand() else {
            panic!("Bad endpoint subcommand for CallServiceEndpointCommand");
        };
        let Some(endpoint) = service.get_endpoint(endpoint_name) else {
            panic!("Bad endpoint name");
        };
        let environment_name: Option<String> = endpoint_matches.bind_field("environment");

        let mut headers = config.headers.clone();
        merge(&mut headers, &service.headers);

        let path = build_path_from_template(&endpoint.path_template, endpoint_matches);
        let mut query_parameters = get_query_parameters_from_args(endpoint, endpoint_matches);

        let query_param_args: Vec<String> = endpoint_matches.bind_field("query_parameters");
        for query_param_arg in query_param_args {
            let (key, value) = parse_query_params_from_arg(query_param_arg.as_str())?;
            query_parameters.insert(key, value);
        }

        Ok(CallServiceEndpointCommand {
            service_name: service_name.to_string(),
            environment_name,
            path,
            query_parameters,
            headers,
            show_body: endpoint_matches.bind_field("show_body"),
            preset: endpoint_matches.bind_field("preset"),
        })
    }

    pub fn execute_command(&self, config: &HtrsConfig) -> Result<HtrsAction, HtrsError> {
        let service = config.get_service(&self.service_name).unwrap();
        let environment = match &self.environment_name {
            Some(environment_name) => service.get_environment(environment_name).unwrap(),
            None => {
                let Some(environment) = service.get_default_environment() else {
                    return Err(HtrsError::new(&format!("No default environment defined for service {}", self.service_name)));
                };
                environment
            }
        };

        let base_url = match Url::parse(format!("https://{}/", environment.host).as_str()) {
            Ok(url) => url,
            Err(e) => return Err(HtrsError::new(format!("Failed to build url from given host: {e}").as_str())),
        };
        let mut url = match base_url.join(self.path.as_str()) {
            Ok(url) => url,
            Err(e) => return Err(HtrsError::new(format!("Failed to build url for endpoint: {e}").as_str())),
        };
        url.set_scheme("http").unwrap();

        Ok(MakeRequest {
            url,
            query_parameters: self.query_parameters.clone(),
            method: Method::GET,
            headers: self.headers.clone(),
            show_body: self.show_body
        })
    }
}

fn get_command_for_service(service: &Service) -> Command {
    let mut command = Command::new(service.name.clone())
        .arg_required_else_help(true)
        .arg(
            Arg::new("environment")
                .value_name("environment name")
                .required(false)
                .help("Environment to target, will use default environment if none specified")
                .long("environment")
                .short('e')
        );

    if let Some(alias) = &service.alias {
        command = command.visible_alias(alias);
    }

    for endpoint in &service.endpoints {
        command = command.subcommand(get_command_for_endpoint(endpoint));
    }

    command
}

fn get_command_for_endpoint(endpoint: &Endpoint) -> Command {
    let mut command = Command::new(endpoint.name.clone())
        .arg(
            Arg::new("environment")
                .value_name("environment")
                .required(false)
                .help("Environment to target, will use default environment if none specified")
                .long("environment")
                .short('e')
        )
        .arg(
            Arg::new("query_parameters")
                .value_name("query param")
                .help("Set a query parameter for the request in the format `name=value`")
                .required(false)
                .action(ArgAction::Append)
                .long("query-param")
                .short('q')
        )
        .arg(
            Arg::new("show_body")
                .help("Print the response body")
                .required(false)
                .num_args(0)
                .long("body")
        )
        .arg(
            Arg::new("preset")
                .help("Use a preset to populate endpoint's parameters")
                .long("preset")
                .short('p')
        );

    let templated_params = get_params_from_path(&endpoint.path_template);
    for templated_param in templated_params {
        command = command.arg(
            Arg::new(&templated_param)
                .allow_hyphen_values(true)
                .long(&templated_param)
                .required_unless_present("preset")
        );
    }

    for param in &endpoint.query_parameters {
        let mut arg = Arg::new(&param.name)
            .allow_hyphen_values(true)
            .long(&param.name);

        if param.required {
            arg = arg.required_unless_present("preset");
        }

        command = command.arg(arg);
    }

    command
}

fn parse_query_params_from_arg(arg: &str) -> Result<(String, String), HtrsBindingError> {
    if let [name, value] = arg.split("=").collect::<Vec<&str>>().as_slice() {
        if !name.is_empty() && !value.is_empty() {
            return Ok((name.to_string(), value.to_string()));
        }
    }

    Err(HtrsBindingError {
        description: format!("Invalid query parameter: {}", arg),
    })
}

fn build_path_from_template(path_template: &str, args: &ArgMatches) -> String {
    let mut path: String = path_template.to_string();
    let template_value_names = get_params_from_path(path_template);
    for template_value_name in &template_value_names {
        let template_value: String = args.bind_field(template_value_name);
        path = path.replace(&format!("{{{}}}", template_value_name.as_str()), &template_value)
    }

    path
}

fn get_query_parameters_from_args(endpoint: &Endpoint, args: &ArgMatches) -> HashMap<String, String> {
    let mut query_parameters = HashMap::new();
    for parameter_name in &endpoint.query_parameters {
        let parameter_value: String = args.bind_field(parameter_name.name.as_str());
        query_parameters.insert(parameter_name.name.to_string(), parameter_value);
    }
    query_parameters
}

fn merge(into: &mut HashMap<String, String>, from: &HashMap<String, String>) {
    for (k, v) in from.iter() {
        into.insert(k.to_string(), v.to_string());
    }
}
