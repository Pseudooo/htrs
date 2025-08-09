use crate::command_args::HeaderCommands::{Clear, Set};
use crate::command_args::ServiceCommands::Environment;
use crate::command_args::{ConfigurationCommands, EnvironmentCommands, RootCommands, ServiceCommands};
use clap::{Arg, ArgMatches, Command};

pub fn map_command(args: ArgMatches) -> RootCommands {
    match args.subcommand() {
        Some(("service", service_matches)) => {
            match service_matches.subcommand() {
                Some(("add", add_service_matches)) => {
                    let Some(name) = add_service_matches.get_one::<String>("name") else {
                        panic!("bad add command")
                    };
                    RootCommands::Service(
                        ServiceCommands::Add {
                            name: name.to_string(),
                        }
                    )
                },
                Some(("remove" | "rm", remove_service_matches)) => {
                    let Some(name) = remove_service_matches.get_one::<String>("name") else {
                        panic!("bad remove command")
                    };
                    RootCommands::Service(
                        ServiceCommands::Remove {
                            name: name.to_string(),
                        }
                    )
                },
                Some(("list" | "ls", _)) => {
                    RootCommands::Service(
                        ServiceCommands::List
                    )
                },
                Some(("environment" | "env", service_environment_matches)) => {
                    match service_environment_matches.subcommand() {
                        Some(("add", add_service_environment_matches)) => {
                            let Some(service_name) = add_service_environment_matches.get_one::<String>("service_name") else {
                                panic!("Add service environment missing service_name")
                            };
                            let Some(environment_name) = add_service_environment_matches.get_one::<String>("environment_name") else {
                                panic!("Add service environment missing environment_name")
                            };
                            let Some(host) = add_service_environment_matches.get_one::<String>("host") else {
                                panic!("Add service environment missing host")
                            };
                            let is_default = add_service_environment_matches.get_flag("default");

                            RootCommands::Service(
                                Environment(EnvironmentCommands::Add {
                                    service_name: service_name.to_string(),
                                    name: environment_name.to_string(),
                                    host: host.to_string(),
                                    default: is_default,
                                })
                            )
                        }
                        Some(("list" | "ls", list_service_environment_matches)) => {
                            let Some(service_name) = list_service_environment_matches.get_one::<String>("service_name") else {
                                panic!("List service environments missing service name");
                            };
                            RootCommands::Service(
                                Environment(EnvironmentCommands::List {
                                    service_name: service_name.to_string(),
                                })
                            )
                        },
                        Some(("remove" | "rm", remove_service_environment_matches)) => {
                            let Some(service_name) = remove_service_environment_matches.get_one::<String>("service_name") else {
                                panic!("Remove service environment missing service name")
                            };
                            let Some(environment_name) = remove_service_environment_matches.get_one::<String>("environment_name") else {
                                panic!("Remove service environment missing environment name")
                            };
                            RootCommands::Service(
                                Environment(
                                    EnvironmentCommands::Remove {
                                        service_name: service_name.to_string(),
                                        environment_name: environment_name.to_string(),
                                    }
                                )
                            )
                        }
                        _ => panic!("Bad service environment command")
                    }
                },
                Some(("configuration" | "config", service_configuration_matches)) => {
                    let Some(service_name) = service_configuration_matches.get_one::<String>("service_name") else {
                        panic!("Service configuration command missing service name");
                    };
                    match service_configuration_matches.subcommand() {
                        Some(("header", service_configure_header_matches)) => {
                            match service_configure_header_matches.subcommand() {
                                Some(("set", service_configuration_set_header_matches)) => {
                                    let Some(header_name) = service_configuration_set_header_matches.get_one::<String>("header_name") else {
                                        panic!("Service configuration set header missing header name");
                                    };
                                    let Some(header_value) = service_configuration_set_header_matches.get_one::<String>("header_value") else {
                                        panic!("Service configuration set header missing header value");
                                    };
                                    RootCommands::Service(
                                        ServiceCommands::Config {
                                            service_name: service_name.to_string(),
                                            config_command: ConfigurationCommands::Header(
                                                Set {
                                                    header: header_name.to_string(),
                                                    value: header_value.to_string(),
                                                }
                                            )
                                        }
                                    )
                                },
                                Some(("clear", service_configuration_clear_header_matches)) => {
                                    let Some(header_name) = service_configuration_clear_header_matches.get_one::<String>("header_name") else {
                                        panic!("Service configuration clear header missing header name");
                                    };
                                    RootCommands::Service(
                                        ServiceCommands::Config {
                                            service_name: service_name.to_string(),
                                            config_command: ConfigurationCommands::Header(
                                                Clear {
                                                    header: header_name.to_string(),
                                                }
                                            )
                                        }
                                    )
                                }
                                _ => panic!("Bad service configuration header command")
                            }
                        },
                        _ => panic!("Bad service configuration command")
                    }
                }
                _ => panic!("Bad service command")
            }
        },

        _ => panic!("scrEEEEch")
    }
}

pub fn get_root_command() -> Command {
    let command = Command::new("htrs")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A flexible http cli client")
        .subcommand(get_service_command());

    command
}

fn get_service_command() -> Command {
    let command = Command::new("service")
        .about("Service configuration commands")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Create a new service")
                .arg(
                    Arg::new("name")
                        .help("Unique name of the service to create")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("remove")
                .visible_alias("rm")
                .about("Remove a service")
                .arg(
                    Arg::new("name")
                        .help("Service name to remove")
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
        );

    command
}

fn get_service_environment_command() -> Command {
    Command::new("environment")
        .visible_alias("env")
        .about("Service environment configuration commands")
        .subcommand(
            Command::new("add")
                .about("Add a new environment to a service")
                .arg(
                    Arg::new("service_name")
                        .value_name("service name")
                        .help("Service to configure")
                        .required(true)
                )
                .arg(
                    Arg::new("environment_name")
                        .value_name("environment name")
                        .help("Unique environment name to add")
                        .required(true)
                )
                .arg(
                    Arg::new("host")
                        .value_name("host")
                        .help("Hostname of the service for this environment")
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
            Command::new("list")
                .visible_alias("ls")
                .about("List all environments for service")
                .arg(
                    Arg::new("service_name")
                        .value_name("service name")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("remove")
                .visible_alias("rm")
                .about("Remove an environment from the service")
                .arg(
                    Arg::new("service_name")
                        .help("Service to remove environment from")
                        .required(true)
                )
                .arg(
                    Arg::new("environment_name")
                        .help("Environment to remove")
                        .required(true)
                )
        )
}

fn get_header_configuration_command() -> Command {
    Command::new("header")
        .about("Configure headers")
        .subcommand(
            Command::new("set")
                .about("Set a header value")
                .arg(
                    Arg::new("header_name")
                        .value_name("header")
                        .help("Header name")
                        .required(true)
                )
                .arg(
                    Arg::new("header_value")
                        .value_name("Header value")
                        .help("Header value")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("clear")
                .about("Clear a header value")
                .arg(
                    Arg::new("header_name")
                        .value_name("header")
                        .help("Header name to clear")
                        .required(true)
                )
        )
}

#[cfg(test)]
mod command_builder_tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn given_valid_add_service_command_then_should_parse_and_map() {
        let args = vec!["htrs", "service", "add", "foo"];

        let result = get_root_command().try_get_matches_from(args);
        let matches = match result {
            Ok(res) => res,
            Err(err) => panic!("Failed to get matches - {err}")
        };
        let mapped_command = map_command(matches);

        let RootCommands::Service(service_command) = mapped_command else {
            panic!("Command was not service command");
        };
        let ServiceCommands::Add { name } = service_command else {
            panic!("Command was not add service command");
        };
        assert_eq!(name, "foo")
    }

    #[rstest]
    #[case("remove")]
    #[case("rm")]
    fn given_valid_remove_service_command_then_should_parse_and_map(#[case] remove_alias: &str) {
        let args = vec!["htrs", "service", remove_alias, "foo"];

        let result = get_root_command().try_get_matches_from(args);
        let matches = match result {
            Ok(res) => res,
            Err(e) => panic!("Failed to get matches - {e}")
        };
        let mapped_command = map_command(matches);

        let RootCommands::Service(service_command) = mapped_command else {
            panic!("Command was not RootCommands::Service");
        };
        let ServiceCommands::Remove { name } = service_command else {
            panic!("Command was not ServiceCommands::Remove")
        };
        assert_eq!(name, "foo");
    }

    #[rstest]
    #[case("list")]
    #[case("ls")]
    fn given_valid_list_services_command_then_should_parse_and_map(#[case] list_alias: &str) {
        let args = vec!["htrs", "service", list_alias];

        let result = get_root_command().try_get_matches_from(args);
        let matches = match result {
            Ok(res) => res,
            Err(e) => panic!("Failed to get matches - {e}")
        };

        let RootCommands::Service(service_command) = map_command(matches) else {
            panic!("Command was not RootCommands::Service")
        };
        assert!(matches!(service_command, ServiceCommands::List));
    }

    #[rstest]
    #[case("environment", true)]
    #[case("environment", false)]
    #[case("env", true)]
    #[case("env", false)]
    fn given_valid_add_service_environment_command_then_should_parse_and_map(
        #[case] environment_alias: &str,
        #[case] set_default: bool
    ) {
        let mut args = vec!["htrs", "service", environment_alias, "add", "foo_service", "foo_environment", "foo_host"];
        if set_default {
            args.push("--default");
        }

        let result = get_root_command().try_get_matches_from(args);
        let matches = match result {
            Ok(res) => res,
            Err(e) => panic!("Failed to get matches - {e}")
        };
        let mapped_command = map_command(matches);

        let RootCommands::Service(service_command) = mapped_command else {
            panic!("Command was not RootCommands::Service");
        };
        let Environment(environment_command) = service_command else {
            panic!("Command was not ServiceCommands::Environment");
        };
        let EnvironmentCommands::Add {
            service_name,
            name,
            host,
            default
        } = environment_command else {
            panic!("Command was not EnvironmentCommands::Add");
        };

        assert_eq!(service_name, "foo_service");
        assert_eq!(name, "foo_environment");
        assert_eq!(host, "foo_host");
        assert_eq!(default, set_default);
    }

    #[rstest]
    #[case("environment", "list")]
    #[case("environment", "ls")]
    #[case("env", "list")]
    #[case("env", "ls")]
    fn given_valid_list_service_environments_command_then_should_parse_and_map(
        #[case] environment_alias: &str,
        #[case] list_alias: &str
    ) {
        let args = vec!["htrs", "service", environment_alias, list_alias, "foo_service"];

        let result = get_root_command().try_get_matches_from(args);
        let matches = match result {
            Ok(res) => res,
            Err(e) => panic!("Failed to get matches - {e}"),
        };
        let mapped_command = map_command(matches);

        let RootCommands::Service(service_command) = mapped_command else {
            panic!("Command was not RootCommands::Service");
        };
        let Environment(environment_command) = service_command else {
            panic!("Command was not ServiceCommands::Environment");
        };
        let EnvironmentCommands::List { service_name } = environment_command else {
            panic!("Command was not EnvironmentCommands::List");
        };
        assert_eq!(service_name, "foo_service");
    }

    #[rstest]
    #[case("environment", "remove")]
    #[case("environment", "rm")]
    #[case("env", "remove")]
    #[case("env", "rm")]
    fn given_valid_remove_service_environment_command_then_should_parse_and_map(
        #[case] environment_alias: &str,
        #[case] remove_alias: &str
    ) {
        let args = vec!["htrs", "service", environment_alias, remove_alias, "foo_service", "foo_environment"];

        let result = get_root_command().try_get_matches_from(args);
        let matches = match result {
            Ok(res) => res,
            Err(e) => panic!("Failed to get matches - {e}")
        };
        let mapped_command = map_command(matches);

        let RootCommands::Service(service_command) = mapped_command else {
            panic!("Command was not RootCommands::Service");
        };
        let Environment(environment_command) = service_command else {
            panic!("Command was not ServiceCommands::Environment");
        };
        let EnvironmentCommands::Remove {
            service_name,
            environment_name
        } = environment_command else {
            panic!("Command was not EnvironmentCommands::Remove");
        };
        assert_eq!(service_name, "foo_service");
        assert_eq!(environment_name, "foo_environment");
    }

    #[rstest]
    #[case("configuration")]
    #[case("config")]
    fn given_valid_service_configuration_set_header_command_then_should_parse_and_map(
        #[case] config_alias: &str
    ) {
        let args = vec!["htrs", "service", config_alias, "foo_service", "header", "set", "foo_header_name", "foo_header_value"];

        let result = get_root_command().try_get_matches_from(args);
        let matches = match result {
            Ok(res) => res,
            Err(e) => panic!("Failed to get matches - {e}"),
        };
        let mapped_command = map_command(matches);

        let RootCommands::Service(service_command) = mapped_command else {
            panic!("Command was not RootCommands::Service")
        };
        let ServiceCommands::Config {
            service_name,
            config_command,
        } = service_command else {
            panic!("Command was not ServiceCommands::Config");
        };
        let ConfigurationCommands::Header(header_command) = config_command;
        let Set {
            header,
            value,
        } = header_command else {
            panic!("Command Configuration was not HeaderCommands::Set");
        };
        assert_eq!(service_name, "foo_service");
        assert_eq!(header, "foo_header_name");
        assert_eq!(value, "foo_header_value");
    }

    #[rstest]
    #[case("configuration")]
    #[case("config")]
    fn given_valid_service_configuration_clear_header_command_then_should_parse_and_map(
        #[case] config_alias: &str
    ) {
        let args = vec!["htrs", "service", config_alias, "foo_service", "header", "clear", "foo_header_name"];

        let result = get_root_command().try_get_matches_from(args);
        let matches = match result {
            Ok(res) => res,
            Err(e) => panic!("Failed to get matches - {e}")
        };
        let mapped_command = map_command(matches);

        let RootCommands::Service(service_command) = mapped_command else {
            panic!("Command was not RootCommands::Service");
        };
        let ServiceCommands::Config {
            service_name,
            config_command,
        } = service_command else {
            panic!("Command was not ServiceCommands::Config");
        };
        let ConfigurationCommands::Header(header_command) = config_command;
        let Clear { header } = header_command else {
            panic!("Command configuration was not HeaderCommands::Clear");
        };
        assert_eq!(service_name, "foo_service");
        assert_eq!(header, "foo_header_name");
    }
}

