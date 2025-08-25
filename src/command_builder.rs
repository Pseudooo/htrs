use crate::command_args::ServiceCommands::Environment;
use crate::command_args::{ConfigurationCommands, EndpointCommands, EnvironmentCommands, HeaderCommands, RootCommands, ServiceCommands};
use crate::commands::call_commands::CallServiceEndpointCommand;
use crate::config::HtrsConfig;
use clap::{Arg, ArgAction, ArgMatches, Command};

pub trait MatchBinding<T> {
    fn bind_field(&self, field_id: &str) -> T;
}

impl MatchBinding<String> for ArgMatches {
    fn bind_field(&self, field_id: &str) -> String {
        let Some(field_value) = self.get_one::<String>(field_id) else {
            panic!("Unexpected binding - no value found");
        };
        field_value.clone()
    }
}

impl MatchBinding<Option<String>> for ArgMatches {
    fn bind_field(&self, field_id: &str) -> Option<String> {
        let Some(value) = self.get_one::<String>(field_id) else {
            return None
        };
        Some(value.clone())
    }
}

impl MatchBinding<bool> for ArgMatches {
    fn bind_field(&self, field_id: &str) -> bool {
        self.get_flag(field_id)
    }
}

impl MatchBinding<Vec<String>> for ArgMatches {
    fn bind_field(&self, field_id: &str) -> Vec<String> {
        let binding = self.get_many::<String>(field_id);
        let Some(binding_value) = binding else {
            return vec![];
        };
        binding_value.cloned().collect()
    }
}

impl RootCommands {
    pub fn bind_from_matches(config: &HtrsConfig, args: &ArgMatches) -> RootCommands {
        match args.subcommand() {
            Some(("service", service_matches)) => {
                RootCommands::Service(
                    ServiceCommands::bind_from_matches(service_matches)
                )
            },
            Some(("call", call_matches)) => {
                RootCommands::Call(
                    CallServiceEndpointCommand::bind_from_matches(config, call_matches)
                )
            },
            Some(("configuration" | "config", config_matches)) => {
                RootCommands::Config(
                    ConfigurationCommands::bind_from_matches(config_matches)
                )
            },
            _ => panic!("Bad subcommand for RootCommands"),
        }
    }
}

impl ServiceCommands {
    pub fn bind_from_matches(args: &ArgMatches) -> ServiceCommands {
        match args.subcommand() {
            Some(("add", add_service_matches)) => {
                let service_name = add_service_matches.bind_field("service_name");
                ServiceCommands::Add {
                    name: service_name,
                }
            },
            Some(("remove" | "rm", remove_service_matches)) => {
                let service_name = remove_service_matches.bind_field("service_name");
                ServiceCommands::Remove {
                    name: service_name,
                }
            },
            Some(("list" | "ls", _)) => {
                ServiceCommands::List
            },
            Some(("environment" | "env", environment_matches)) => {
                Environment(
                    EnvironmentCommands::bind_from_matches(environment_matches)
                )
            },
            Some(("configuration" | "config", config_matches)) => {
                ServiceCommands::Config {
                    service_name: config_matches.bind_field("service_name"),
                    config_command: ConfigurationCommands::bind_from_matches(config_matches),
                }
            },
            Some(("endpoint", endpoint_matches)) => {
                ServiceCommands::Endpoint {
                    service_name: endpoint_matches.bind_field("service_name"),
                    command: EndpointCommands::bind_from_matches(endpoint_matches),
                }
            }
            _ => panic!("Bad subcommand for ServiceCommands"),
        }
    }
}

impl EnvironmentCommands {
    pub fn bind_from_matches(args: &ArgMatches) -> EnvironmentCommands {
        match args.subcommand() {
            Some(("add", add_environment_matches)) => {
                EnvironmentCommands::Add {
                    service_name: add_environment_matches.bind_field("service_name"),
                    name: add_environment_matches.bind_field("environment_name"),
                    host: add_environment_matches.bind_field("host"),
                    default: add_environment_matches.bind_field("default"),
                }
            },
            Some(("list" | "ls", list_environment_matches)) => {
                EnvironmentCommands::List {
                    service_name: list_environment_matches.bind_field("service_name"),
                }
            },
            Some(("remove" | "rm", remove_environment_matches)) => {
                EnvironmentCommands::Remove {
                    service_name: remove_environment_matches.bind_field("service_name"),
                    environment_name: remove_environment_matches.bind_field("environment_name"),
                }
            },
            _ => panic!("Bad subcommand for EnvironmentCommands"),
        }
    }
}

impl EndpointCommands {
    fn bind_from_matches(args: &ArgMatches) -> EndpointCommands {
        match args.subcommand() {
            Some(("add", add_endpoint_matches)) => {
                EndpointCommands::Add {
                    name: add_endpoint_matches.bind_field("endpoint_name"),
                    path_template: add_endpoint_matches.bind_field("path_template"),
                    query_parameters: add_endpoint_matches.bind_field("query_parameters"),
                }
            },
            Some(("list", _)) => {
                EndpointCommands::List
            },
            Some(("remove" | "rm", remove_endpoint_matches)) => {
                EndpointCommands::Remove {
                    name: remove_endpoint_matches.bind_field("endpoint_name"),
                }
            }
            _ => panic!("Bad subcommand for EndpointCommands"),
        }
    }
}

impl ConfigurationCommands {
    pub fn bind_from_matches(args: &ArgMatches) -> ConfigurationCommands {
        match args.subcommand() {
            Some(("header", header_matches)) => {
                ConfigurationCommands::Header(
                    HeaderCommands::bind_from_matches(header_matches)
                )
            }
            _ => panic!("Bad subcommand for ConfigurationCommands"),
        }
    }
}

impl HeaderCommands {
    pub fn bind_from_matches(args: &ArgMatches) -> HeaderCommands {
        match args.subcommand() {
            Some(("set", set_header_matches)) => {
                HeaderCommands::Set {
                    header: set_header_matches.bind_field("header_name"),
                    value: set_header_matches.bind_field("header_value"),
                }
            },
            Some(("clear", clear_header_matches)) => {
                HeaderCommands::Clear {
                    header: clear_header_matches.bind_field("header_name"),
                }
            },
            _ => panic!("Bad subcommand for HeaderCommands"),
        }
    }
}

pub fn get_root_command(config: &HtrsConfig) -> Command {
    let command = Command::new("htrs")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A flexible http cli client")
        .subcommand(get_service_command())
        .subcommand(CallServiceEndpointCommand::get_command(&config))
        .subcommand(
            Command::new("configuration")
                .visible_alias("config")
                .about("Global configuration")
                .subcommand(get_header_configuration_command())
        );

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
                    Arg::new("service_name")
                        .help("Unique name of the service to create")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("remove")
                .visible_alias("rm")
                .about("Remove a service")
                .arg(
                    Arg::new("service_name")
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
        )
        .subcommand(get_endpoint_command());

    command
}

fn get_service_environment_command() -> Command {
    Command::new("environment")
        .visible_alias("env")
        .about("Service environment configuration commands")
        .arg_required_else_help(true)
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

fn get_endpoint_command() -> Command {
    Command::new("endpoint")
        .about("Configure service endpoints")
        .arg_required_else_help(true)
        .arg(
            Arg::new("service_name")
                .value_name("service name")
                .required(true)
                .help("Service name to configure endpoints for")
        )
        .subcommand(
            Command::new("add")
                .about("Add a new service endpoint")
                .arg(
                    Arg::new("endpoint_name")
                        .value_name("endpoint name")
                        .required(true)
                        .help("The unique endpoint name")
                )
                .arg(
                    Arg::new("path_template")
                        .value_name("path template")
                        .required(true)
                        .help("The templated path of endpoint")
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
                .about("List all endpoints for a service")
        )
        .subcommand(
            Command::new("remove")
                .visible_alias("rm")
                .about("Remove an endpoint from a service")
                .arg(
                    Arg::new("endpoint_name")
                        .value_name("endpoint name")
                        .required(true)
                        .help("The endpoint name to remove")
                )
        )
}

#[cfg(test)]
mod command_builder_tests {
    use super::*;
    use crate::command_args::EndpointCommands;
    use crate::config::{Endpoint, ServiceConfig, ServiceEnvironmentConfig};
    use rstest::rstest;
    use ConfigurationCommands::Header;

    fn bind_command_from_vec(args: Vec<&str>) -> RootCommands {
        bind_command_from_vec_with_config(HtrsConfig::new(), args)
    }

    fn bind_command_from_vec_with_config(config: HtrsConfig, args: Vec<&str>) -> RootCommands {
        let result = get_root_command(&config).try_get_matches_from(args);
        let matches = match result {
            Ok(res) => res,
            Err(e) => panic!("Failed to get matches - {e}")
        };
        RootCommands::bind_from_matches(&config, &matches)
    }

    #[test]
    fn given_valid_add_service_command_then_should_parse_and_map() {
        let args = vec!["htrs", "service", "add", "foo"];

        let command = bind_command_from_vec(args);

        let RootCommands::Service(service_command) = command else {
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

        let command = bind_command_from_vec(args);

        let RootCommands::Service(service_command) = command else {
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

        let command = bind_command_from_vec(args);

        let RootCommands::Service(service_command) = command else {
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

        let command = bind_command_from_vec(args);

        let RootCommands::Service(service_command) = command else {
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

        let command = bind_command_from_vec(args);

        let RootCommands::Service(service_command) = command else {
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

        let command = bind_command_from_vec(args);

        let RootCommands::Service(service_command) = command else {
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

        let command = bind_command_from_vec(args);

        let RootCommands::Service(service_command) = command else {
            panic!("Command was not RootCommands::Service")
        };
        let ServiceCommands::Config {
            service_name,
            config_command,
        } = service_command else {
            panic!("Command was not ServiceCommands::Config");
        };
        let Header(header_command) = config_command;
        let HeaderCommands::Set {
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

        let command = bind_command_from_vec(args);

        let RootCommands::Service(service_command) = command else {
            panic!("Command was not RootCommands::Service");
        };
        let ServiceCommands::Config {
            service_name,
            config_command,
        } = service_command else {
            panic!("Command was not ServiceCommands::Config");
        };
        let Header(header_command) = config_command;
        let HeaderCommands::Clear { header } = header_command else {
            panic!("Command configuration was not HeaderCommands::Clear");
        };
        assert_eq!(service_name, "foo_service");
        assert_eq!(header, "foo_header_name");
    }

    #[rstest]
    #[case("configuration")]
    #[case("config")]
    fn given_valid_configuration_command_when_set_header_then_should_parse_and_map(
        #[case] config_alias: &str
    ) {
        let args = vec!["htrs", config_alias, "header", "set", "foo_header_name", "foo_header_value"];

        let command = bind_command_from_vec(args);

        let RootCommands::Config(config_command) = command else {
            panic!("Command was not RootCommands::Config");
        };
        let Header(header_command) = config_command;
        let HeaderCommands::Set {
            header,
            value
        } = header_command else {
            panic!("Command was not HeaderCommands::Set");
        };
        assert_eq!(header, "foo_header_name");
        assert_eq!(value, "foo_header_value");
    }

    #[rstest]
    #[case("configuration")]
    #[case("config")]
    fn given_valid_configuration_command_when_clear_header_then_should_parse_and_map(
        #[case] config_alias: &str
    ) {
        let args = vec!["htrs", config_alias, "header", "clear", "foo_header_name"];

        let command = bind_command_from_vec(args);

        let RootCommands::Config(config_command) = command else {
            panic!("Command was not RootCommands::Config");
        };
        let Header(header_command) = config_command;
        let HeaderCommands::Clear {
            header
        } = header_command else {
            panic!("Command was not HeaderCommands::Set");
        };
        assert_eq!(header, "foo_header_name");
    }

    #[test]
    fn given_valid_service_endpoint_command_when_add_endpoint_then_should_parse_and_map() {
        let args = vec!["htrs", "service", "endpoint", "foo_service", "add", "foo_endpoint", "/foo/my/path", "-q", "query_param1", "--query-param", "query_param2"];

        let command = bind_command_from_vec(args);

        let RootCommands::Service(service_command) = command else {
            panic!("Command was not RootCommands::Service");
        };
        let ServiceCommands::Endpoint { service_name, command: endpoint_command} = service_command else {
            panic!("Command was not ServiceCommands::Endpoint");
        };
        let EndpointCommands::Add {
            name: endpoint_name,
            path_template,
            query_parameters
        } = endpoint_command else {
            panic!("Command was not EndpointCommands::Add");
        };
        assert_eq!(service_name, "foo_service");
        assert_eq!(endpoint_name, "foo_endpoint");
        assert_eq!(path_template, "/foo/my/path");
        assert_eq!(query_parameters, vec!["query_param1", "query_param2"]);
    }

    #[test]
    fn given_valid_service_endpoint_command_when_list_endpoints_then_should_parse_and_map() {
        let args = vec!["htrs", "service", "endpoint", "foo_service", "list"];

        let command = bind_command_from_vec(args);

        let RootCommands::Service(service_command) = command else {
            panic!("Command was not RootCommands::Service");
        };
        let ServiceCommands::Endpoint { service_name, command: endpoint_command} = service_command else {
            panic!("Command was not ServiceCommands::Endpoint");
        };
        assert!(matches!(endpoint_command, EndpointCommands::List));
        assert_eq!(service_name, "foo_service");
    }

    #[rstest]
    #[case("remove")]
    #[case("rm")]
    fn given_valid_service_endpoint_command_when_remove_endpoint_then_should_parse_and_map(
        #[case] remove_alias: &str
    ) {
        let args = vec!["htrs", "service", "endpoint", "foo_service", remove_alias, "foo_endpoint"];

        let command = bind_command_from_vec(args);

        let RootCommands::Service(service_command) = command else {
            panic!("Command was not RootCommands::Service");
        };
        let ServiceCommands::Endpoint { service_name, command: endpoint_command} = service_command else {
            panic!("Command was not ServiceCommands::Endpoint");
        };
        let EndpointCommands::Remove { name: endpoint_name } = endpoint_command else {
            panic!("Command was not EndpointCommands::Remove");
        };
        assert_eq!(service_name, "foo_service");
        assert_eq!(endpoint_name, "foo_endpoint");
    }

    fn given_valid_call_endpoint_command_with_known_endpoint_then_should_parse_and_map() {
        let args = vec!["htrs", "call", "foo_service", "foo_endpoint", "add", "foo_endpoint", "--foo_template", "foo_template", "--foo_value", "foo_value"];
        let environment = ServiceEnvironmentConfig::new(
            "foo_environment".to_string(),
            "foo.host.com".to_string(),
            true);
        let endpoint = Endpoint {
            name: "foo_endpoint".to_string(),
            path_template: "/my/{foo_template}/path".to_string(),
            query_parameters: vec!["foo_value".to_string()],
        };
        let mut service = ServiceConfig::new("foo_service".to_string());
        service.endpoints.push(endpoint);
        service.environments.push(environment);
        let mut config = HtrsConfig::new();
        config.services.push(service);

        let command = bind_command_from_vec_with_config(config, args);

        let RootCommands::Call(options) = command else {
            panic!("Command was not RootCommands::Call");
        };
    }
}

