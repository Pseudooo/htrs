use crate::command_builder::MatchBinding;
use crate::config::{Endpoint, HtrsConfig};
use crate::outcomes::HtrsAction::MakeRequest;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgMatches, Command};
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{Method, Url};
use std::collections::HashMap;
use std::str::FromStr;

pub struct CallServiceEndpointCommand {
    pub service_name: String,
    pub environment_name: Option<String>,
    pub path: Url,
    pub query_parameters: HashMap<String, String>,
}

impl CallServiceEndpointCommand {
    pub fn get_command(config: &HtrsConfig) -> Command {
        let mut command = Command::new("call")
            .about("Call a service endpoint")
            .arg(
                Arg::new("environment_name")
                    .value_name("environment name")
                    .required(false)
                    .help("Environment to target, will use default environment if none specified")
            );

        for service in &config.services {
            let mut service_command = Command::new(service.name.clone());
            for endpoint in &service.endpoints {
                let mut endpoint_command = Command::new(endpoint.name.clone());

                let templated_params = get_path_template_params(&endpoint.path_template);
                for templated_param in templated_params {
                    endpoint_command = endpoint_command.arg(
                        Arg::new(&templated_param)
                            .long(&templated_param)
                            .required(true)
                    );
                }

                for param in &endpoint.query_parameters {
                    endpoint_command = endpoint_command.arg(
                        Arg::new(param)
                            .long(param)
                            .required(true)
                    )
                }
                service_command = service_command.subcommand(endpoint_command);
            }
            command = command.subcommand(service_command);
        }

        return command;
    }

    pub fn bind_from_matches(config: &HtrsConfig, args: &ArgMatches) -> CallServiceEndpointCommand {
        let environment_name: Option<String> = args.bind_field("environment_name");

        let Some((service_name, service_matches)) = args.subcommand() else {
            panic!("Bad service subcommand for CallServiceEndpointCommand");
        };
        let Some(service) = config.find_service_config(service_name) else {
            panic!("Bad service name");
        };

        let Some((endpoint_name, endpoint_matches)) = service_matches.subcommand() else {
            panic!("Bad endpoint subcommand for CallServiceEndpointCommand");
        };
        let Some(endpoint) = service.find_endpoint(endpoint_name) else {
            panic!("Bad endpoint name");
        };

        let path = build_path_from_template(&endpoint.path_template, endpoint_matches);
        let query_parameters = get_query_parameters_from_args(endpoint, endpoint_matches);

        CallServiceEndpointCommand {
            service_name: service_name.to_string(),
            environment_name,
            path,
            query_parameters,
        }
    }

    pub fn execute_command(&self, config: &HtrsConfig) -> Result<HtrsAction, HtrsError> {
        let service = config.find_service_config(&self.service_name).unwrap();
        let environment = match &self.environment_name {
            Some(environment_name) => service.find_environment(&environment_name).unwrap(),
            None => {
                let Some(environment) = service.find_default_environment() else {
                    return Err(HtrsError::new(&format!("No default environment defined for service {}", self.service_name)));
                };
                environment
            }
        };

        let url_string = format!("https://{}/{}", environment.host, self.path);
        let url = match Url::parse(&url_string) {
            Ok(url) => url,
            Err(e) => return Err(HtrsError::new(&format!("Failed to build url: {e}")))
        };

        Ok(MakeRequest {
            url,
            query_parameters: self.query_parameters.clone(),
            method: Method::GET,
        })
    }
}

fn get_path_template_params(path_template: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\{([A-Za-z0-1]|_|-)+}").unwrap();
    }
    RE.find_iter(path_template)
        .filter_map(|s| s.as_str().parse().ok())
        .map(|s: String| s[1..s.len() - 1].to_string())
        .collect()
}

fn build_path_from_template(path_template: &str, args: &ArgMatches) -> Url {
    let mut path: String = path_template.to_string();
    let template_value_names = get_path_template_params(path_template);
    for template_value_name in template_value_names {
        let template_value: String = args.bind_field(&template_value_name);
        path = path.replace(&format!("{{{}}}", template_value_name.as_str()), &template_value)
    }

    return Url::from_str(&path).unwrap();
}

fn get_query_parameters_from_args(endpoint: &Endpoint, args: &ArgMatches) -> HashMap<String, String> {
    let mut query_parameters = HashMap::new();
    for parameter_name in &endpoint.query_parameters {
        let parameter_value: String = args.bind_field(&parameter_name);
        query_parameters.insert(parameter_name.to_string(), parameter_value);
    }
    return query_parameters;
}
