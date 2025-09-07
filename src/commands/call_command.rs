use crate::command_builder::MatchBinding;
use crate::config::{Endpoint, HtrsConfig};
use crate::outcomes::HtrsAction::MakeRequest;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgAction, ArgMatches, Command};
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{Method, Url};
use std::collections::HashMap;

pub struct CallServiceEndpointCommand {
    pub service_name: String,
    pub environment_name: Option<String>,
    pub path: String,
    pub query_parameters: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub show_body: bool,
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
            )
            .arg_required_else_help(true);

        for service in &config.services {
            let mut service_command = Command::new(service.name.clone())
                .arg_required_else_help(true);
            if let Some(alias) = &service.alias {
                service_command = service_command.visible_alias(alias);
            }

            for endpoint in &service.endpoints {
                let mut endpoint_command = Command::new(endpoint.name.clone())
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
                    );

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
        let Some(service) = config.get_service(service_name) else {
            panic!("Bad service name");
        };

        let Some((endpoint_name, endpoint_matches)) = service_matches.subcommand() else {
            panic!("Bad endpoint subcommand for CallServiceEndpointCommand");
        };
        let Some(endpoint) = service.get_endpoint(endpoint_name) else {
            panic!("Bad endpoint name");
        };

        let mut headers = config.headers.clone();
        merge(&mut headers, &service.headers);

        let path = build_path_from_template(&endpoint.path_template, endpoint_matches);
        let mut query_parameters = get_query_parameters_from_args(endpoint, endpoint_matches);

        let query_param_args: Vec<String> = endpoint_matches.bind_field("query_parameters");
        for query_param_arg in query_param_args {
            match query_param_arg.split("=").collect::<Vec<&str>>().as_slice() {
                [key, value] => {
                    query_parameters.insert(key.to_string(), value.to_string());
                }
                _ => panic!("Query parameter was not in format `key=value`: {}", query_param_arg),
            }
        }

        CallServiceEndpointCommand {
            service_name: service_name.to_string(),
            environment_name,
            path,
            query_parameters,
            headers,
            show_body: args.bind_field("show_body"),
        }
    }

    pub fn execute_command(&self, config: &HtrsConfig) -> Result<HtrsAction, HtrsError> {
        let service = config.get_service(&self.service_name).unwrap();
        let environment = match &self.environment_name {
            Some(environment_name) => service.get_environment(&environment_name).unwrap(),
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
        let url = match base_url.join(self.path.as_str()) {
            Ok(url) => url,
            Err(e) => return Err(HtrsError::new(format!("Failed to build url for endpoint: {e}").as_str())),
        };

        Ok(MakeRequest {
            url,
            query_parameters: self.query_parameters.clone(),
            method: Method::GET,
            headers: self.headers.clone(),
            show_body: self.show_body
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

fn build_path_from_template(path_template: &str, args: &ArgMatches) -> String {
    let mut path: String = path_template.to_string();
    let template_value_names = get_path_template_params(path_template);
    for template_value_name in &template_value_names {
        let template_value: String = args.bind_field(&template_value_name);
        path = path.replace(&format!("{{{}}}", template_value_name.as_str()), &template_value)
    }

    path
}

fn get_query_parameters_from_args(endpoint: &Endpoint, args: &ArgMatches) -> HashMap<String, String> {
    let mut query_parameters = HashMap::new();
    for parameter_name in &endpoint.query_parameters {
        let parameter_value: String = args.bind_field(&parameter_name);
        query_parameters.insert(parameter_name.to_string(), parameter_value);
    }
    return query_parameters;
}

fn merge(into: &mut HashMap<String, String>, from: &HashMap<String, String>) {
    for (k, v) in from.iter() {
        into.insert(k.to_string(), v.to_string());
    }
}

#[cfg(test)]
mod call_command_execution_tests {
    use super::*;
    use crate::command_args::RootCommands;
    use crate::command_args::RootCommands::Call;
    use crate::command_builder::get_root_command;
    use crate::test_helpers::{HtrsConfigBuilder, HtrsServiceBuilder};
    use clap::Error;
    use rstest::rstest;

    fn parse_and_bind(config: HtrsConfig, args: Vec<&str>) -> Result<RootCommands, Error> {
        let command = get_root_command(&config);
        let matches = command.try_get_matches_from(args)?;
        Ok(RootCommands::bind_from_matches(&config, &matches))
    }

    #[rstest]
    #[case(true)]
    #[case(false)]
    fn given_service_with_known_endpoint_when_no_parameters_then_parse_and_map(
        #[case] print_body: bool
    ) {
        let config = HtrsConfigBuilder::new()
            .with_service(
                HtrsServiceBuilder::new()
                    .with_name("foo_service")
                    .with_endpoint("foo_endpoint", "/my/path", vec![])
            )
            .build();
        let mut args = vec!["htrs", "call", "foo_service", "foo_endpoint"];
        if print_body {
            args.push("--body")
        }

        let result = parse_and_bind(config, args).unwrap();

        let Call(command) = result else {
            panic!("Parsed command was not RootCommands::Call");
        };
        assert_eq!(command.service_name, "foo_service");
        assert_eq!(command.environment_name, None);
        assert_eq!(command.path.as_str(), "/my/path");
        assert_eq!(command.query_parameters.len(), 0);
        assert_eq!(command.show_body, print_body);
    }

    #[test]
    fn given_service_with_known_endpoint_when_required_path_template_parameters_provided_then_parse_and_map() {
        let config = HtrsConfigBuilder::new()
            .with_service(
                HtrsServiceBuilder::new()
                    .with_name("foo_service")
                    .with_endpoint("foo_endpoint", "/my/templated/path/{template_param}", vec![])
            )
            .build();
        let args = vec!["htrs", "call", "foo_service", "foo_endpoint", "--template_param", "foo_value"];

        let result = parse_and_bind(config, args).unwrap();

        let Call(command) = result else {
            panic!("Parsed command was not RootCommands::Call");
        };
        assert_eq!(command.service_name, "foo_service");
        assert_eq!(command.environment_name, None);
        assert_eq!(command.path.as_str(), "/my/templated/path/foo_value");
        assert_eq!(command.query_parameters.len(), 0);
    }

    #[test]
    fn given_service_with_known_endpoint_when_required_path_params_not_provided_then_should_error() {
        let config = HtrsConfigBuilder::new()
            .with_service(
                HtrsServiceBuilder::new()
                    .with_name("foo_service")
                    .with_endpoint("foo_endpoint", "/my/templated/path/{template_param}", vec![])
            )
            .build();
        let args = vec!["htrs", "call", "foo_service", "foo_endpoint"];

        let result = parse_and_bind(config, args);
        assert!(result.is_err(), "Result was not an error");
    }

    #[test]
    fn given_service_with_known_endpoint_when_required_query_params_provided_then_parse_and_map() {
        let config = HtrsConfigBuilder::new()
            .with_service(
                HtrsServiceBuilder::new()
                    .with_name("foo_service")
                    .with_endpoint("foo_endpoint", "/my/path", vec!["foo_query_param"])
            )
            .build();
        let args = vec!["htrs", "call", "foo_service", "foo_endpoint", "--foo_query_param", "foo_value"];

        let result = parse_and_bind(config, args).unwrap();

        let Call(command) = result else {
            panic!("Parsed command was not RootCommands::Call");
        };
        assert_eq!(command.service_name, "foo_service");
        assert_eq!(command.environment_name, None);
        assert_eq!(command.path.as_str(), "/my/path");
        assert_eq!(command.query_parameters.len(), 1);
        assert!(command.query_parameters.contains_key("foo_query_param"), "Query parameters did not contain expected value");
        assert_eq!(command.query_parameters["foo_query_param"], "foo_value");
    }

    #[test]
    fn given_service_with_known_endpoint_when_required_query_params_not_provided_then_parse_and_map() {
        let config = HtrsConfigBuilder::new()
            .with_service(
                HtrsServiceBuilder::new()
                    .with_name("foo_service")
                    .with_endpoint("foo_endpoint", "/my/path", vec!["foo_query_param"])
            )
            .build();
        let args = vec!["htrs", "call", "foo_service", "foo_endpoint"];

        let result = parse_and_bind(config, args);
        assert!(result.is_err(), "Result was not an error");
    }

    #[test]
    fn given_service_with_known_endpoint_when_query_params_provided_then_parse_and_map() {
        let config = HtrsConfigBuilder::new()
            .with_service(
                HtrsServiceBuilder::new()
                    .with_name("foo_service")
                    .with_endpoint("foo_endpoint", "/my/path", vec![])
            )
            .build();
        let args = vec!["htrs", "call", "foo_service", "foo_endpoint", "--query-param", "param1=value1", "-q", "param2=value2"];

        let result = parse_and_bind(config, args);

        assert!(result.is_ok(), "Result was an error: {}", result.err().unwrap());
        let Call(command) = result.unwrap() else {
            panic!("Parsed command was not RootCommands::Call");
        };
        assert_eq!(command.service_name, "foo_service");
        assert_eq!(command.environment_name, None);
        assert_eq!(command.path.as_str(), "/my/path");
        assert_eq!(command.query_parameters.len(), 2);
        assert_eq!(command.query_parameters["param1"], "value1");
        assert_eq!(command.query_parameters["param2"], "value2");
    }
}
