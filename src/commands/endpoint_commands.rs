use crate::command_builder::MatchBinding;
use crate::config::{Endpoint, HtrsConfig};
use crate::outcomes::HtrsAction::{PrintDialogue, UpdateConfig};
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgAction, ArgMatches, Command};

pub enum EndpointCommand {
    Add {
        service: String,
        name: String,
        path_template: String,
        query_parameters: Vec<String>,
    },
    Remove {
        service: String,
        name: String,
    },
    List {
        service: String,
    },
}

impl EndpointCommand {
    pub fn get_command() -> Command {
        Command::new("endpoint")
            .arg_required_else_help(true)
            .arg(
                Arg::new("service")
                    .value_name("service name")
                    .required(true)
                    .help("Service name to configure endpoint for")
            )
            .subcommand(
                Command::new("add")
                    .about("Add a new service endpoint")
                    .arg(
                        Arg::new("name")
                            .value_name("endpoint name")
                            .help("The unique endpoint name")
                            .required(true)
                    )
                    .arg(
                        Arg::new("path_template")
                            .value_name("path template")
                            .help("Templated path of the endpoint")
                            .required(true)
                    )
                    .arg(
                        Arg::new("query_parameters")
                            .long("query-param")
                            .short('q')
                            .required(false)
                            .action(ArgAction::Append)
                            .help("Query parameter for endpoint")
                    )
            )
            .subcommand(
                Command::new("list")
                    .visible_alias("ls")
                    .about("List all endpoints for a service")
            )
            .subcommand(
                Command::new("remove")
                    .visible_alias("rm")
                    .arg(
                        Arg::new("name")
                            .value_name("endpoint name")
                            .help("Endpoint name to be removed")
                            .required(true)
                    )
            )
    }

    pub fn bind_from_matches(args: &ArgMatches) -> EndpointCommand {
        match args.subcommand() {
            Some(("add", add_matches)) => {
                EndpointCommand::Add {
                    service: args.bind_field("service"),
                    name: add_matches.bind_field("name"),
                    path_template: add_matches.bind_field("path_template"),
                    query_parameters: add_matches.bind_field("query_parameters"),
                }
            },
            Some(("list" | "ls", _)) => {
                EndpointCommand::List {
                    service: args.bind_field("service"),
                }
            },
            Some(("remove" | "rm", remove_matches)) => {
                EndpointCommand::Remove {
                    service: args.bind_field("service"),
                    name: remove_matches.bind_field("name"),
                }
            },
            _ => panic!("Bad subcommand for EndpointCommand"),
        }
    }

    pub fn execute_command(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        match self {
            EndpointCommand::Add {
                service,
                name,
                path_template,
                query_parameters
            } => {
                add_new_endpoint(config, service, name, path_template, query_parameters)
            },
            EndpointCommand::List { service } => list_endpoints(config, service),
            EndpointCommand::Remove { service, name } => remove_endpoint(config, service, name),
        }
    }
}

fn add_new_endpoint(config: &mut HtrsConfig, service: &str, name: &str, path_template: &str, query_parameters: &Vec<String>) -> Result<HtrsAction, HtrsError> {
    let Some(service) = config.get_service_mut(service) else {
        return Err(HtrsError::new(format!("Service '{service}' not found").as_str()));
    };
    if service.get_endpoint(name).is_some() {
        return Err(HtrsError::new(format!("Service '{}' already has an endpoint named '{name}'", service.name).as_str()))
    }

    service.endpoints.push(Endpoint {
        name: name.to_string(),
        path_template: path_template.to_string(),
        query_parameters: query_parameters.clone(),
    });
    Ok(UpdateConfig)
}

fn list_endpoints(config: &HtrsConfig, service: &str) -> Result<HtrsAction, HtrsError> {
    let Some(service) = config.get_service(service) else {
        return Err(HtrsError::new(format!("Service '{service}' not found").as_str()))
    };

    if service.endpoints.len() == 0 {
        return Ok(PrintDialogue("No endpoints defined".to_string()))
    }

    let dialogue = service.endpoints.iter()
        .map(|e| format!(" - {}", e.name))
        .collect::<Vec<String>>()
        .join("\n");
    Ok(PrintDialogue(dialogue))
}

fn remove_endpoint(config: &mut HtrsConfig, service: &str, name: &str) -> Result<HtrsAction, HtrsError> {
    let Some(service) = config.get_service_mut(service) else {
        return Err(HtrsError::new(format!("Service '{service}' not found").as_str()))
    };
    match service.remove_endpoint(name) {
        true => Ok(UpdateConfig),
        false => Err(HtrsError::new(format!("Service '{}' has no endpoint named '{name}'", service.name).as_str()))
    }
}

#[cfg(test)]
mod endpoint_command_binding_tests {
    use super::*;
    use clap::Error;
    use rstest::rstest;

    fn get_parsed_command(args: Vec<&str>) -> Result<EndpointCommand, Error> {
        let command = EndpointCommand::get_command();
        let matches = command.try_get_matches_from(args)?;
        Ok(EndpointCommand::bind_from_matches(&matches))
    }

    #[test]
    fn given_valid_add_endpoint_command_when_no_query_params_then_should_parse_and_map() {
        let args = vec!["htrs", "foo_service", "add", "foo_endpoint", "foo_path"];

        let result = get_parsed_command(args);

        assert!(result.is_ok(), "{}", result.err().unwrap());
        let EndpointCommand::Add {
            service, name, path_template, query_parameters
        } = result.unwrap() else {
            panic!("Command was not EndpointCommand::Add")
        };
        assert_eq!(service, "foo_service");
        assert_eq!(name, "foo_endpoint");
        assert_eq!(path_template, "foo_path");
        assert_eq!(query_parameters, vec![] as Vec<String>);
    }

    #[test]
    fn given_valid_add_endpoint_command_when_query_params_then_should_parse_and_map() {
        let args = vec!["htrs", "foo_service", "add", "foo_endpoint", "foo_path", "-q", "param1", "--query-param", "param2"];

        let result = get_parsed_command(args);

        assert!(result.is_ok(), "{}", result.err().unwrap());
        let EndpointCommand::Add {
            service, name, path_template, query_parameters
        } = result.unwrap() else {
            panic!("Command was not EndpointCommand::Add");
        };
        assert_eq!(service, "foo_service");
        assert_eq!(name, "foo_endpoint");
        assert_eq!(path_template, "foo_path");
        assert_eq!(query_parameters, vec!["param1".to_string(), "param2".to_string()]);
    }

    #[rstest]
    #[case("list")]
    #[case("ls")]
    fn given_valid_list_endpoints_command_then_should_parse_and_map(
        #[case] list_cmd: &str,
    ) {
        let args = vec!["htrs", "foo_service", list_cmd];

        let result = get_parsed_command(args);

        assert!(result.is_ok(), "{}", result.err().unwrap());
        let EndpointCommand::List { service } = result.unwrap() else {
            panic!("Command was not EndpointCommand::List");
        };
        assert_eq!(service, "foo_service");
    }

    #[rstest]
    #[case("remove")]
    #[case("rm")]
    fn given_valid_remove_endpoint_command_then_should_parse_and_map(
        #[case] remove_cmd: &str,
    ) {
        let args = vec!["htrs", "foo_service", remove_cmd, "foo_endpoint"];

        let result = get_parsed_command(args);

        assert!(result.is_ok(), "{}", result.err().unwrap());
        let EndpointCommand::Remove {
            service, name
        } = result.unwrap() else {
            panic!("Command was not EndpointCommand::Remove");
        };
        assert_eq!(service, "foo_service");
        assert_eq!(name, "foo_endpoint");
    }
}

#[cfg(test)]
mod endpoint_command_execution_tests {
    use super::*;
    use crate::test_helpers::{HtrsConfigBuilder, HtrsServiceBuilder};

    #[test]
    fn given_known_service_when_add_endpoint_then_should_add_endpoint() {
        let mut config = HtrsConfigBuilder::new()
            .with_service(
                HtrsServiceBuilder::new()
                    .with_name("foo_service")
            )
            .build();
        let command = EndpointCommand::Add {
            service: "foo_service".to_string(),
            name: "foo_endpoint".to_string(),
            path_template: "/foo/path".to_string(),
            query_parameters: vec!["param".to_string()],
        };

        let result = command.execute_command(&mut config);

        assert!(result.is_ok(), "{}", result.err().unwrap());
        assert!(matches!(result.unwrap(), UpdateConfig), "Returned action was not HtrsAction::UpdateConfig");

        let new_endpoint = config.get_service("foo_service")
            .unwrap()
            .get_endpoint("foo_endpoint");
        assert!(new_endpoint.is_some(), "New endpoint not added to config");
        assert_eq!(new_endpoint.unwrap().name, "foo_endpoint");
        assert_eq!(new_endpoint.unwrap().path_template, "/foo/path");
        assert_eq!(new_endpoint.unwrap().query_parameters, vec!["param".to_string()]);
    }

    #[test]
    fn given_unknown_service_when_add_endpoint_then_should_error() {
        let mut config = HtrsConfigBuilder::new()
            .build();
        let command = EndpointCommand::Add {
            service: "foo_service".to_string(),
            name: "foo_endpoint".to_string(),
            path_template: "/foo/path".to_string(),
            query_parameters: vec!["param".to_string()],
        };

        let result = command.execute_command(&mut config);

        assert!(result.is_err(), "{}", "Result was not error");
    }

    #[test]
    fn given_known_service_when_add_existing_endpoint_then_should_error() {
        let mut config = HtrsConfigBuilder::new()
            .with_service(
                HtrsServiceBuilder::new()
                    .with_name("foo_service")
                    .with_endpoint("foo_endpoint", "/my/path", vec![])
            )
            .build();
        let command = EndpointCommand::Add {
            service: "foo_service".to_string(),
            name: "foo_endpoint".to_string(),
            path_template: "/foo/path".to_string(),
            query_parameters: vec![],
        };

        let result = command.execute_command(&mut config);

        assert!(result.is_err(), "{}", "Result was not error");
        assert_eq!(config.get_service("foo_service").unwrap().endpoints.len(), 1);
    }

    #[test]
    fn given_known_service_and_known_endpoint_when_remove_then_should_remove() {
        let mut config = HtrsConfigBuilder::new()
            .with_service(
                HtrsServiceBuilder::new()
                    .with_name("foo_service")
                    .with_endpoint("foo_endpoint", "/my/path", vec![])
            )
            .build();
        let command = EndpointCommand::Remove {
            service: "foo_service".to_string(),
            name: "foo_endpoint".to_string(),
        };

        let result = command.execute_command(&mut config);

        assert!(result.is_ok(), "{}", result.err().unwrap());
        assert!(matches!(result.unwrap(), UpdateConfig), "Returned action was not HtrsAction::UpdateConfig");
        assert_eq!(config.get_service("foo_service").unwrap().endpoints.len(), 0);
    }

    #[test]
    fn given_unknown_service_when_remove_endpoint_then_should_error() {
        let mut config = HtrsConfigBuilder::new()
            .build();
        let command = EndpointCommand::Remove {
            service: "foo_service".to_string(),
            name: "foo_endpoint".to_string(),
        };

        let result = command.execute_command(&mut config);

        assert!(result.is_err(), "Result was not error");
    }

    #[test]
    fn given_known_service_and_unknown_endpoint_when_remove_endpoint_then_should_error() {
        let mut config = HtrsConfigBuilder::new()
            .with_service(
                HtrsServiceBuilder::new()
                    .with_name("foo_service")
            )
            .build();
        let command = EndpointCommand::Remove {
            service: "foo_service".to_string(),
            name: "foo_endpoint".to_string(),
        };

        let result = command.execute_command(&mut config);

        assert!(result.is_err(), "Result was not error");
    }

    #[test]
    fn given_list_endpoints_command_then_should_list() {
        let mut config = HtrsConfigBuilder::new()
            .with_service(
                HtrsServiceBuilder::new()
                    .with_name("foo_service")
                    .with_endpoint("foo_endpoint", "/my/path", vec![])
            )
            .build();
        let command = EndpointCommand::List {
            service: "foo_service".to_string(),
        };

        let result = command.execute_command(&mut config);

        assert!(result.is_ok(), "{}", result.err().unwrap());
        let PrintDialogue(dialogue) = result.unwrap() else {
            panic!("Returned action was not HtrsAction::PrintDialogue");
        };
        assert!(dialogue.contains(" - foo_endpoint"), "Dialogue did not contain endpoint");
    }

    #[test]
    fn given_known_service_with_no_endpoints_when_list_endpoints_should_list_none() {
        let mut config = HtrsConfigBuilder::new()
            .with_service(
                HtrsServiceBuilder::new()
                    .with_name("foo_service")
            )
            .build();
        let command = EndpointCommand::List {
            service: "foo_service".to_string(),
        };

        let result = command.execute_command(&mut config);
        assert!(result.is_ok(), "Result was error");
        let PrintDialogue(dialogue) = result.unwrap() else {
            panic!("Returned action was not HtrsAction::PrintDialogue");
        };
        assert_eq!(dialogue, "No endpoints defined");
    }

    #[test]
    fn given_unknown_service_when_list_endpoints_then_should_error() {
        let mut config = HtrsConfigBuilder::new()
            .build();
        let command = EndpointCommand::List {
            service: "foo_service".to_string(),
        };

        let result = command.execute_command(&mut config);

        assert!(result.is_err(), "Result was not error");
    }
}
