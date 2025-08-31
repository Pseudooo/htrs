use crate::command_builder::MatchBinding;
use crate::config::{HtrsConfig, ServiceEnvironmentConfig};
use crate::outcomes::HtrsAction::{PrintDialogue, UpdateConfig};
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgMatches, Command};

pub enum EnvironmentCommand {
    Add {
        service: String,
        name: String,
        alias: Option<String>,
        host: String,
        default: bool
    },
    Remove {
        service: String,
        environment: String,
    },
    List {
        service: String,
    }
}

impl EnvironmentCommand {
    pub fn get_command() -> Command {
        Command::new("environment")
            .visible_alias("env")
            .arg_required_else_help(true)
            .subcommand(
                Command::new("add")
                    .about("Add a new environment to a service")
                    .arg(
                        Arg::new("service")
                            .value_name("service name")
                            .help("The name or alias of the service")
                            .required(true)
                    )
                    .arg(
                        Arg::new("name")
                            .value_name("environment name")
                            .help("Unique name of the environment to create")
                            .required(true)
                    )
                    .arg(
                        Arg::new("alias")
                            .value_name("alias")
                            .help("Alias for the environment")
                            .long("alias")
                            .short('a')
                            .required(false)
                    )
                    .arg(
                        Arg::new("host")
                            .value_name("host")
                            .help("Hostname for the service in the environment")
                            .required(true)
                    )
                    .arg(
                        Arg::new("default")
                            .long("default")
                            .num_args(0)
                            .required(false)
                            .help("Set as the default environment for the service")
                    )
            )
            .subcommand(
                Command::new("remove")
                    .visible_alias("rm")
                    .about("Remove an environment")
                    .arg(
                        Arg::new("service")
                            .value_name("service name")
                            .help("The name or alias of the service")
                            .required(true)
                    )
                    .arg(
                        Arg::new("environment")
                            .value_name("environment name")
                            .help("The environment name or alias to remove")
                            .required(true)
                    )
            )
            .subcommand(
                Command::new("list")
                    .visible_alias("ls")
                    .about("List a service's environments")
                    .arg(
                        Arg::new("service")
                            .value_name("service name")
                            .help("The name or alias of the service")
                            .required(true)
                    )
            )
    }

    pub fn bind_from_matches(args: &ArgMatches) -> EnvironmentCommand {
        match args.subcommand() {
            Some(("add", add_matches)) => {
                EnvironmentCommand::Add {
                    service: add_matches.bind_field("service"),
                    name: add_matches.bind_field("name"),
                    alias: add_matches.bind_field("alias"),
                    host: add_matches.bind_field("host"),
                    default: add_matches.bind_field("default"),
                }
            },
            Some(("remove" | "rm", remove_matches)) => {
                EnvironmentCommand::Remove {
                    service: remove_matches.bind_field("service"),
                    environment: remove_matches.bind_field("environment"),
                }
            },
            Some(("list" | "ls", list_matches)) => {
                EnvironmentCommand::List {
                    service: list_matches.bind_field("service"),
                }
            }
            _ => panic!("Bad subcommand for EnvironmentCommand"),
        }
    }

    pub fn execute_command(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        match self {
            EnvironmentCommand::Add {
                service, name, alias, host, default
            } => {
                add_new_environment(config, service, name, alias, host, default)
            },
            EnvironmentCommand::Remove {
                service, environment
            } => {
                remove_environment(config, service, environment)
            },
            EnvironmentCommand::List { service } => list_environments(config, service),
        }
    }
}

fn add_new_environment(config: &mut HtrsConfig, service_name: &str, name: &str, alias: &Option<String>, host: &str, default: &bool) -> Result<HtrsAction, HtrsError> {
    let Some(service) = config.get_service_mut(service_name) else {
        return Err(HtrsError::new(format!("Service '{service_name}' not found").as_str()))
    };
    if service.get_environment(name).is_some() {
        return Err(HtrsError::new(format!("Service '{service_name}' already has an environment with name or alias '{name}'").as_str()))
    }
    if let Some(alias) = alias {
        if service.get_environment(alias).is_some() {
            return Err(HtrsError::new(format!("Service '{service_name}' already has an environment with name or alias '{alias}'").as_str()))
        }
    }

    service.environments.push(
        ServiceEnvironmentConfig::new(
            name.to_string(),
            alias.clone(),
            host.to_string(),
            default.clone()
        )
    );
    Ok(UpdateConfig)
}

fn remove_environment(config: &mut HtrsConfig, service_name: &str, environment_name: &str) -> Result<HtrsAction, HtrsError> {
    let Some(service) = config.get_service_mut(service_name) else {
        return Err(HtrsError::new(format!("Service '{service_name}' not found").as_str()))
    };
    if service.remove_environment(environment_name) {
        return Err(HtrsError::new(format!("Service '{service_name}' has no environment '{environment_name}'").as_str()));
    }
    Ok(UpdateConfig)
}

fn list_environments(config: &HtrsConfig, service_name: &str) -> Result<HtrsAction, HtrsError> {
    let Some(service) = config.get_service(service_name) else {
        return Err(HtrsError::new(format!("Service '{service_name}' not found").as_str()));
    };

    let dialogue = service.environments.iter()
        .map(|e| match &e.alias {
            Some(alias) => format!("- {} ({})", e.name, alias),
            None => format!("- {}", e.name),
        })
        .collect::<Vec<String>>()
        .join("\n");
    Ok(PrintDialogue(dialogue))
}

#[cfg(test)]
mod environment_command_binding_tests {
    use super::*;
    use clap::Error;
    use rstest::rstest;

    fn get_parsed_command(args: Vec<&str>) -> Result<EnvironmentCommand, Error> {
        let command = EnvironmentCommand::get_command();
        let matches = command.try_get_matches_from(args)?;
        Ok(EnvironmentCommand::bind_from_matches(&matches))
    }

    #[rstest]
    #[case(true)]
    #[case(false)]
    fn given_valid_add_environment_command_when_no_alias_then_should_parse_and_map(
        #[case] is_default: bool,
    ) {
        let mut args = vec!["htrs", "add", "foo_service", "foo_environment", "host.com"];
        if is_default {
            args.push("--default")
        }

        let result = get_parsed_command(args);

        assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
        let EnvironmentCommand::Add {
            service,  name, alias, host, default
        } = result.unwrap() else {
            panic!("Command is not EnvironmentCommand::Add");
        };
        assert_eq!(service, "foo_service");
        assert_eq!(name, "foo_environment");
        assert_eq!(alias, None);
        assert_eq!(host, "host.com");
        assert_eq!(default, is_default);
    }

    #[rstest]
    #[case(true)]
    #[case(false)]
    fn given_valid_add_environment_command_when_alias_then_should_parse_and_map(
        #[case] is_default: bool,
    ) {
        let mut args = vec!["htrs", "add", "foo_service", "foo_environment", "host.com", "--alias", "foo_alias"];
        if is_default {
            args.push("--default");
        }

        let result = get_parsed_command(args);

        assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
        let EnvironmentCommand::Add {
            service, name, alias, host, default
        } = result.unwrap() else {
            panic!("Command is not EnvironmentCommand::Add");
        };
        assert_eq!(service, "foo_service");
        assert_eq!(name, "foo_environment");
        assert_eq!(alias, Some("foo_alias".to_string()));
        assert_eq!(host, "host.com");
        assert_eq!(default, is_default);
    }

    #[rstest]
    #[case("remove")]
    #[case("rm")]
    fn given_valid_remove_environment_command_then_should_parse_and_map(
        #[case] remove_cmd: &str,
    ) {
        let args = vec!["htrs", remove_cmd, "foo_service", "foo_environment"];

        let result = get_parsed_command(args);

        assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
        let EnvironmentCommand::Remove {
            service, environment
        } = result.unwrap() else {
            panic!("Command is not EnvironmentCommand::Remove")
        };
        assert_eq!(service, "foo_service");
        assert_eq!(environment, "foo_environment");
    }

    #[rstest]
    #[case("list")]
    #[case("ls")]
    fn given_valid_list_environments_command_then_should_parse_and_map(
        #[case] list_cmd: &str,
    ) {
        let args = vec!["htrs", list_cmd, "foo_service"];

        let result = get_parsed_command(args);

        assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
        let EnvironmentCommand::List {
            service
        } = result.unwrap() else {
            panic!("Command is not EnvironmentCommand::List");
        };
        assert_eq!(service, "foo_service");
    }
}
