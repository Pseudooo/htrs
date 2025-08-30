use crate::command_args::ConfigurationCommands::Header;
use crate::command_args::HeaderCommands::{Clear, Set};
use crate::command_args::ServiceCommands::{Add, Environment, Remove};
use crate::command_args::{ConfigurationCommands, EndpointCommands, EnvironmentCommands, ServiceCommands};
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
    Config {
        service: String,
        config_command: ConfigurationCommands,
    },
    Environment(EnvironmentCommands),
    Endpoint {
        service: String,
        command: EndpointCommands,
    }
}

impl ServiceCommand {
    pub fn get_command(config: &HtrsConfig) -> Command {
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
            Some(("configuration" | "config", config_matches)) => {
                let service = config_matches.bind_field("service_name");
                ServiceCommand::Config {
                    service,
                    config_command: ConfigurationCommands::bind_from_matches(config_matches),
                }
            }
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
        todo!("Not done yet");
    }
}

pub fn execute_service_command(config: &mut HtrsConfig, cmd: &ServiceCommands) -> Result<HtrsAction, HtrsError> {
    match cmd {
        Add { name } => {
            for service in config.services.iter() {
                if name.eq(service.name.as_str()) {
                    return Err(HtrsError::new(&format!("Service \"{name}\" already exists")))
                }
            }

            config.services.push(ServiceConfig::new(name.clone(), None));
            Ok(UpdateConfig)
        },

        Remove { name } => {
            if config.service_defined(name) {
                config.services.retain(|x| !x.name.eq(name));
                Ok(UpdateConfig)
            } else {
                Err(HtrsError::new(&format!("Service \"{name}\" does not exist")))
            }
        }

        ServiceCommands::List => {
            if config.services.len() == 0 {
                return Ok(PrintDialogue("No services exist".to_string()));
            }

            let dialogue = config.services
                .iter()
                .map(|service| format!(" - {}", service.name))
                .collect::<Vec<String>>()
                .join("\n");
            Ok(PrintDialogue(dialogue))
        },

        ServiceCommands::Config { service_name, config_command } => {
            let Some(service) = config.find_service_config_mut(&service_name) else {
                return Err(HtrsError::new(&format!("Service \"{}\" does not exist", service_name)))
            };

            let Header(header_cmd) = config_command;
            match header_cmd {
                Set { header, value } => {
                    service.headers.insert(header.clone(), value.clone());
                    Ok(UpdateConfig)
                },
                Clear { header } => {
                    if config.headers.remove(header) == None {
                        Err(HtrsError::new(&format!("No header `{}` defined", header)))
                    } else {
                        Ok(UpdateConfig)
                    }
                },
            }
        },

        Environment(env_command) => {
            execute_environment_command(config, env_command)
        }

        ServiceCommands::Endpoint { service_name, command} => {
            execute_endpoint_command(config, service_name, command)
        }
    }
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
            let Some(service) = config.find_service_config(&service_name) else {
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
mod service_command_tests {
    use super::*;
    use clap::Error;

    fn get_parsed_command(args: Vec<&str>) -> Result<ServiceCommand, Error> {
        let command = ServiceCommand::get_command(&HtrsConfig::new());
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
}
