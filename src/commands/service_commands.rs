use crate::command_args::{EndpointCommands, EnvironmentCommands};
use crate::command_builder::{get_endpoint_command, get_header_configuration_command, get_service_environment_command, MatchBinding};
use crate::config::{Endpoint, HtrsConfig, ServiceConfig, ServiceEnvironmentConfig};
use crate::outcomes::HtrsAction::{PrintDialogue, UpdateConfig};
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgMatches, Command};

pub enum ServiceCommand {
    Add {
        name: String,
        alias: Option<String>,
    },
    Remove {
        name: String,
    },
    List,
    Environment(EnvironmentCommands),
    Endpoint {
        service: String,
        command: EndpointCommands,
    }
}

impl ServiceCommand {
    pub fn get_command() -> Command {
        Command::new("service")
            .about("Service commands")
            .arg_required_else_help(true)
            .subcommand(
                Command::new("add")
                    .about("Create a new service")
                    .arg(
                        Arg::new("name")
                            .help("Unique name of the service to create")
                            .required(true)
                    )
                    .arg(
                        Arg::new("alias")
                            .help("Unique alias for the service")
                            .long("alias")
                            .short('a')
                            .required(false)
                    )
            )
            .subcommand(
                Command::new("remove")
                    .visible_alias("rm")
                    .about("Remove a service")
                    .arg(
                        Arg::new("name")
                            .help("Service name or alias to remove")
                            .required(true)
                    )
            )
            .subcommand(
                Command::new("list")
                    .visible_alias("ls")
                    .about("List all services")
            )
            .subcommand(get_service_environment_command())
            .subcommand(
                Command::new("configuration")
                    .visible_alias("config")
                    .about("Service configuration")
                    .arg(
                        Arg::new("service_name")
                            .value_name("Service name")
                            .help("Service name to configure")
                            .required(true)
                    )
                    .subcommand(get_header_configuration_command())
            )
            .subcommand(get_endpoint_command())
    }

    pub fn bind_from_matches(args: &ArgMatches) -> ServiceCommand {
        match args.subcommand() {
            Some(("add", add_matches)) => {
                let name = add_matches.bind_field("name");
                let alias = add_matches.bind_field("alias");
                ServiceCommand::Add {
                    name,
                    alias
                }
            },
            Some(("remove" | "rm", remove_matches)) => {
                let name = remove_matches.bind_field("name");
                ServiceCommand::Remove {
                    name,
                }
            }
            Some(("list" | "ls", _)) => {
                ServiceCommand::List
            },
            Some(("environment" | "env", environment_matches)) => {
                ServiceCommand::Environment(
                    EnvironmentCommands::bind_from_matches(environment_matches),
                )
            },
            Some(("endpoint", endpoint_matches)) => {
                let service = endpoint_matches.bind_field("service_name");
                ServiceCommand::Endpoint {
                    service,
                    command: EndpointCommands::bind_from_matches(endpoint_matches),
                }
            }
            _ => panic!("Bad subcommand given for ServiceCommand"),
        }
    }

    pub fn execute_command(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        match self {
            ServiceCommand::Add { name, alias } => add_new_service(config, name, alias),
            ServiceCommand::Remove { name } => remove_service(config, name),
            ServiceCommand::List => list_services(config),
            ServiceCommand::Environment(environment_command) => execute_environment_command(config, environment_command),
            ServiceCommand::Endpoint { service, command } => execute_endpoint_command(config, service, command),
        }
    }
}

fn add_new_service(config: &mut HtrsConfig, name: &str, alias: &Option<String>) -> Result<HtrsAction, HtrsError> {
    if config.get_service(name).is_some() {
        return Err(HtrsError::new(format!("A service already exists with the name or alias '{name}'").as_str()));
    }
    if let Some(alias) = alias {
        if config.get_service(alias).is_some() {
            return Err(HtrsError::new(format!("A service already exists with the name or alias '{alias}").as_str()));
        }
    }

    config.services.push(ServiceConfig::new(name.to_string(), alias.clone()));
    Ok(UpdateConfig)
}

fn remove_service(config: &mut HtrsConfig, name: &str) -> Result<HtrsAction, HtrsError> {
    match config.remove_service(name) {
        true => Ok(UpdateConfig),
        false => Err(HtrsError::new(format!("Service '{}' not found", name).as_str())),
    }
}

fn list_services(config: &HtrsConfig) -> Result<HtrsAction, HtrsError> {
    if config.services.len() == 0 {
        return Ok(PrintDialogue("No services defined".to_string()));
    }

    let dialogue = config.services
        .iter()
        .map(|s| match &s.alias {
            Some(alias) => format!(" - {} ({})", s.name, alias),
            None => format!(" - {}", s.name),
        })
        .collect::<Vec<String>>()
        .join("\n");
    Ok(PrintDialogue(dialogue))
}

fn execute_environment_command(config: &mut HtrsConfig, cmd: &EnvironmentCommands) -> Result<HtrsAction, HtrsError> {
    match cmd {
        EnvironmentCommands::Add { service_name, name: environment_name, host, default } => {
            let Some(service) = config.find_service_config_mut(&service_name) else {
                return Err(HtrsError::new(&format!("Service `{}` not found", service_name)))
            };

            if service.environment_exists(&environment_name) {
                return Err(HtrsError::new(&format!("Service `{}` already has an environmnt called `{}`", service_name, environment_name)))
            }

            if *default {
                if let Some(curr_default_environment) = service.find_default_environment_mut() {
                    curr_default_environment.default = false;
                }
            }

            let new_env = ServiceEnvironmentConfig::new(
                environment_name.clone(),
                None,
                host.clone(),
                *default,
            );
            service.environments.push(new_env);
            Ok(UpdateConfig)
        },

        EnvironmentCommands::List { service_name } => {
            let Some(service) = config.get_service(&service_name) else {
                return Err(HtrsError::new(&format!("Service `{}` not found", service_name)))
            };

            if service.environments.len() == 0 {
                return Ok(PrintDialogue(format!("No environments defined for `{}`", service_name)));
            }

            let dialogue = service.environments.iter()
                .map(|env| match env.default {
                    true => format!(" - {} (default)", env.name),
                    false => format!(" - {}", env.name),
                })
                .collect::<Vec<String>>()
                .join("\n");
            Ok(PrintDialogue(dialogue))
        },

        EnvironmentCommands::Remove { service_name, environment_name } => {
            let Some(service) = config.find_service_config_mut(&service_name) else {
                return Err(HtrsError::new(&format!("Service `{}` not found", service_name)));
            };

            if !service.environment_exists(&environment_name) {
                return Err(HtrsError::new(&format!("Service `{}` has no environment `{}`", service_name, environment_name)));
            }

            service.remove_environment(environment_name);
            Ok(UpdateConfig)
        }
    }
}

fn execute_endpoint_command(config: &mut HtrsConfig, service_name: &String, cmd: &EndpointCommands) -> Result<HtrsAction, HtrsError> {
    let Some(service) = config.find_service_config_mut(&service_name) else {
        return Err(HtrsError::new(&format!("Service `{}` not found", service_name)));
    };
    match cmd {
        EndpointCommands::Add { name, path_template, query_parameters } => {
            if service.endpoint_exists(&name) {
                return Err(HtrsError::new(&format!("Endpoint `{}` already exists", name)));
            }

            service.endpoints.push(Endpoint {
                name: name.to_string(),
                path_template: path_template.to_string(),
                query_parameters: query_parameters.clone(),
            });
            Ok(UpdateConfig)
        },
        EndpointCommands::List => {
            if service.endpoints.len() == 0 {
                return Ok(PrintDialogue(format!("No endpoints defined for `{}`", service_name)));
            }

            let dialogue = service.endpoints
                .iter()
                .map(|service| format!(" - {}", service.name))
                .collect::<Vec<String>>()
                .join("\n");
            Ok(PrintDialogue(dialogue))
        },
        EndpointCommands::Remove { name } => {
            let success = service.remove_endpoint(&name);
            match success {
                true => Ok(UpdateConfig),
                false => Err(HtrsError::new(&format!("Endpoint `{}` not found", name))),
            }
        }
    }
}

#[cfg(test)]
mod service_command_binding_tests {
    use super::*;
    use clap::Error;
    use rstest::rstest;

    fn get_parsed_command(args: Vec<&str>) -> Result<ServiceCommand, Error> {
        let command = ServiceCommand::get_command();
        let matches = command.try_get_matches_from(args)?;
        Ok(ServiceCommand::bind_from_matches(&matches))
    }

    #[test]
    fn given_valid_add_service_command_when_no_alias_then_should_parse_and_map() {
        let args = vec!["htrs", "add", "foo_service"];

        let result = get_parsed_command(args);

        assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
        let ServiceCommand::Add{
            name, alias
        } = result.unwrap() else {
            panic!("Command was not ServiceCommand::Add")
        };
        assert_eq!(name, "foo_service");
        assert_eq!(alias, None);
    }

    #[test]
    fn given_valid_add_service_command_when_alias_then_should_parse_and_map() {
        let args = vec!["htrs", "add", "foo_service", "--alias", "foo_alias"];

        let result = get_parsed_command(args);

        assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
        let ServiceCommand::Add{name, alias} = result.unwrap() else {
            panic!("Command was not ServiceCommand::Add");
        };
        assert_eq!(name, "foo_service");
        assert_eq!(alias, Some("foo_alias".to_string()));
    }

    #[rstest]
    #[case("remove")]
    #[case("rm")]
    fn given_valid_remove_service_command_then_should_parse_and_map(
        #[case] remove_cmd: &str
    ) {
        let args = vec!["htrs", remove_cmd, "foo_service"];

        let result = get_parsed_command(args);

        assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
        let ServiceCommand::Remove{name} = result.unwrap() else {
            panic!("Command was not ServiceCommand::Remove");
        };
        assert_eq!(name, "foo_service");
    }

    #[rstest]
    #[case("list")]
    #[case("ls")]
    fn given_valid_list_services_command_then_should_parse_and_map(
        #[case] list_cmd: &str
    ) {
        let args = vec!["htrs", list_cmd];

        let result = get_parsed_command(args);

        assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
        assert!(matches!(result.ok().unwrap(), ServiceCommand::List), "Command was not ServiceCommand::List");
    }
}

#[cfg(test)]
mod service_command_execution_tests {
    use super::*;
    use crate::test_helpers::{HtrsConfigBuilder, HtrsServiceBuilder};
    use rstest::rstest;

    #[rstest]
    #[case(Some("foo_alias".to_string()))]
    #[case(None)]
    fn given_add_service_command_when_valid_then_should_add_service(
        #[case] alias: Option<String>
    ) {
        let mut config = HtrsConfigBuilder::new()
            .build();
        let command = ServiceCommand::Add {
            name: "foo_service".to_string(),
            alias: alias.clone(),
        };

        let result = command.execute_command(&mut config);

        assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
        let action = result.unwrap();
        assert!(matches!(action, HtrsAction::UpdateConfig), "Returned action was not HtrsAction::UpdateConfig");

        let new_service = config.get_service("foo_service");
        assert!(new_service.is_some(), "Service with name 'foo_service' not found");
        assert_eq!(new_service.unwrap().name, "foo_service");
        assert_eq!(new_service.unwrap().alias, alias);
    }

    #[rstest]
    #[case("existing_name", "new_alias")]
    #[case("new_name", "existing_alias")]
    fn given_add_service_command_when_service_exists_then_should_error(
        #[case] name: String,
        #[case] alias: String,
    ) {
        let mut config = HtrsConfigBuilder::new()
            .with_service(
                HtrsServiceBuilder::new()
                    .with_name("existing_name")
                    .with_alias("existing_alias")
            )
            .build();
        let command = ServiceCommand::Add {
            name,
            alias: Some(alias),
        };

        let result = command.execute_command(&mut config);

        assert!(result.is_err());
        assert_eq!(config.services.len(), 1);
    }

    #[rstest]
    #[case("foo_service")]
    #[case("foo_alias")]
    fn given_known_service_when_execute_remove_command_then_should_remove(
        #[case] service: String
    ) {
        let mut config = HtrsConfigBuilder::new()
            .with_service(
                HtrsServiceBuilder::new()
                    .with_name("foo_service")
                    .with_alias("foo_alias")
            )
            .build();
        let command = ServiceCommand::Remove {
            name: service
        };

        let result = command.execute_command(&mut config);

        assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
        assert_eq!(config.services.len(), 0);
    }

    #[test]
    fn given_unknown_service_when_remove_then_should_error() {
        let mut config = HtrsConfigBuilder::new()
            .with_service(
                HtrsServiceBuilder::new()
                    .with_name("foo_service")
                    .with_alias("foo_alias")
            )
            .build();
        let command = ServiceCommand::Remove {
            name: "foo".to_string(),
        };

        let result = command.execute_command(&mut config);

        assert!(result.is_err());
    }

    #[test]
    fn given_known_services_when_execute_list_command_then_should_list() {
        let mut config = HtrsConfigBuilder::new()
            .with_service(
                HtrsServiceBuilder::new()
                    .with_name("service1")
                    .with_alias("alias1")
            )
            .with_service(
                HtrsServiceBuilder::new()
                    .with_name("service2")
            )
            .build();
        let command = ServiceCommand::List;

        let result = command.execute_command(&mut config);

        assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
        let PrintDialogue(text) = result.unwrap() else {
            panic!("Action was not HtrsAction::PrintDialogue");
        };
        assert!(text.contains(" - service1 (alias1)"), "Returned text did not contain service1");
        assert!(text.contains(" - service2"), "Returned text did not contain service2");
    }
}
