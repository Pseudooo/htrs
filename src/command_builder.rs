use crate::command_args::{ConfigurationCommands, HeaderCommands, RootCommands};
use crate::commands::call_command::CallServiceEndpointCommand;
use crate::commands::service_commands::ServiceCommand;
use crate::config::HtrsConfig;
use clap::{Arg, ArgMatches, Command};

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
                    ServiceCommand::bind_from_matches(service_matches)
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
        .subcommand(ServiceCommand::get_command())
        .subcommand(CallServiceEndpointCommand::get_command(config))
        .subcommand(
            Command::new("configuration")
                .visible_alias("config")
                .about("Global configuration")
                .subcommand(get_header_configuration_command())
        );

    command
}

pub fn get_header_configuration_command() -> Command {
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
}

