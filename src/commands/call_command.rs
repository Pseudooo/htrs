use crate::command_builder::MatchBinding;
use crate::config::{Endpoint, HtrsConfig};
use crate::outcomes::HtrsAction::MakeRequest;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgMatches, Command};
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{Method, Url};
use std::collections::HashMap;

pub struct CallServiceEndpointCommand {
    pub service_name: String,
    pub environment_name: Option<String>,
    pub path: String,
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
            )
            .arg_required_else_help(true);

        for service in &config.services {
            let mut service_command = Command::new(service.name.clone())
                .arg_required_else_help(true);
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

#[cfg(test)]
mod command_builder_tests {
    use super::*;
    use crate::command_args::RootCommands;
    use crate::command_args::RootCommands::Call;
    use crate::command_builder::get_root_command;
    use crate::config::{ServiceConfig, ServiceEnvironmentConfig};
    use clap::Error;

    struct HtrsConfigBuilder {
        pub services: Vec<ServiceConfig>,
    }

    impl HtrsConfigBuilder {
        fn new() -> HtrsConfigBuilder {
            HtrsConfigBuilder {
                services: Vec::new(),
            }
        }

        fn with_service(mut self, service_builder: HtrsServiceBuilder) -> HtrsConfigBuilder {
            let service = service_builder.build();
            self.services.push(service);
            self
        }

        fn build(self) -> HtrsConfig {
            let mut config = HtrsConfig::new();
            config.services = self.services.clone();
            return config;
        }
    }

    struct HtrsServiceBuilder {
        pub name: Option<String>,
        pub environments: Vec<ServiceEnvironmentConfig>,
        pub endpoints: Vec<Endpoint>
    }

    impl HtrsServiceBuilder {
        fn new() -> Self {
            Self {
                name: None,
                environments: Vec::new(),
                endpoints: Vec::new(),
            }
        }

        fn with_name(mut self, name: &str) -> HtrsServiceBuilder {
            self.name = Some(name.to_string());
            self
        }

        fn with_endpoint(mut self, name: &str, path_template: &str, query_parameters: Vec<&str>) -> HtrsServiceBuilder {
            let endpoint = Endpoint {
                name: name.to_string(),
                path_template: path_template.to_string(),
                query_parameters: query_parameters.iter().map(|s| s.to_string()).collect(),
            };
            self.endpoints.push(endpoint);
            self
        }

        fn build(self) -> ServiceConfig {
            let Some(name) = &self.name else {
                panic!("No name specified for service");
            };

            ServiceConfig {
                name: name.clone(),
                alias: None,
                headers: HashMap::new(),
                endpoints: self.endpoints.clone(),
                environments: self.environments.clone(),
            }
        }
    }


    fn parse_and_bind(config: HtrsConfig, args: Vec<&str>) -> Result<RootCommands, Error> {
        let command = get_root_command(&config);
        let matches = command.try_get_matches_from(args)?;
       Ok(RootCommands::bind_from_matches(&config, &matches))
    }

    #[test]
    fn given_service_with_known_endpoint_when_no_parameters_then_parse_and_map() {
        let config = HtrsConfigBuilder::new()
            .with_service(
                HtrsServiceBuilder::new()
                    .with_name("foo_service")
                    .with_endpoint("foo_endpoint", "/my/path", vec![])
            )
            .build();
        let args = vec!["htrs", "call", "foo_service", "foo_endpoint"];

        let result = parse_and_bind(config, args).unwrap();

        let Call(command) = result else {
            panic!("Parsed command was not RootCommands::Call");
        };
        assert_eq!(command.service_name, "foo_service");
        assert_eq!(command.environment_name, None);
        assert_eq!(command.path.as_str(), "/my/path");
        assert_eq!(command.query_parameters.len(), 0);
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
}
