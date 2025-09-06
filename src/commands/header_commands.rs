use crate::command_builder::MatchBinding;
use crate::config::HtrsConfig;
use crate::outcomes::HtrsAction::UpdateConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgMatches, Command};

pub enum HeaderCommand{
    Set {
        header_name: String,
        header_value: String,
    },
    Clear {
        header_name: String,
    },
}

impl HeaderCommand {
    pub fn get_command() -> Command {
        Command::new("header")
            .about("Manage global headers")
            .arg_required_else_help(true)
            .subcommand(
                Command::new("set")
                    .about("Set a header value")
                    .arg(
                        Arg::new("header_name")
                            .help("The header name")
                            .value_name("header name")
                            .required(true)
                    )
                    .arg(
                        Arg::new("header_value")
                            .help("The header value")
                            .value_name("header value")
                            .required(true)
                    )
            )
            .subcommand(
                Command::new("clear")
                    .about("Clear a header value")
                    .arg(
                        Arg::new("header_name")
                            .help("The header name")
                            .value_name("header name")
                            .required(true)
                    )
            )
    }

    pub fn bind_from_matches(args: &ArgMatches) -> HeaderCommand {
        match args.subcommand() {
            Some(("set", set_matches)) => {
                HeaderCommand::Set {
                    header_name: set_matches.bind_field("header_name"),
                    header_value: set_matches.bind_field("header_value"),
                }
            },
            Some(("clear", clear_matches)) => {
                HeaderCommand::Clear {
                    header_name: clear_matches.bind_field("header_name"),
                }
            },
            _ => panic!("Bad subcommand for HeaderCommand"),
        }
    }

    pub fn execute_command(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        match self {
            HeaderCommand::Set { header_name, header_value } => {
                config.set_header(header_name.clone(), header_value.clone());
            }
            HeaderCommand::Clear { header_name } => {
                config.clear_header(header_name.clone());
            }
        }
        Ok(UpdateConfig)
    }
}

#[cfg(test)]
mod header_command_binding_tests {
    use super::*;
    use clap::Error;

    fn get_parsed_command(args: Vec<&str>) -> Result<HeaderCommand, Error> {
        let command = HeaderCommand::get_command();
        let matches = command.try_get_matches_from(args)?;
        Ok(HeaderCommand::bind_from_matches(&matches))
    }

    #[test]
    fn given_valid_set_header_command_then_should_parse_and_map() {
        let args = vec!["htrs", "set", "header_name", "header_value"];

        let result = get_parsed_command(args);

        assert!(result.is_ok(), "Result was not ok: {}", result.err().unwrap());
        let HeaderCommand::Set {
            header_name,
            header_value} = result.unwrap() else {
            panic!("Command was not HeaderCommand::Set");
        };
        assert_eq!(header_name, "header_name");
        assert_eq!(header_value, "header_value");
    }

    #[test]
    fn given_valid_clear_header_command_then_should_parse_and_map() {
        let args = vec!["htrs", "clear", "header_name"];

        let result = get_parsed_command(args);

        assert!(result.is_ok(), "Result was not ok: {}", result.err().unwrap());
        let HeaderCommand::Clear {
            header_name,
        } = result.unwrap() else {
            panic!("Command was not HeaderCommand::Clear");
        };
        assert_eq!(header_name, "header_name");
    }
}

#[cfg(test)]
mod header_command_execution_tests {
    use super::*;
    use crate::test_helpers::HtrsConfigBuilder;

    #[test]
    fn given_set_header_command_when_valid_then_should_set_header() {
        let mut config = HtrsConfigBuilder::new()
            .build();
        let command = HeaderCommand::Set {
            header_name: "header_name".to_string(),
            header_value: "header_value".to_string(),
        };

        let result = command.execute_command(&mut config);

        assert!(result.is_ok(), "Result was not ok: {}", result.err().unwrap());
        assert!(matches!(result.unwrap(), HtrsAction::UpdateConfig), "Returned action was not HtrsAction::UpdateConfig");

        let new_header = config.get_header_value("header_name".to_string());
        assert!(matches!(new_header, Some(_)), "Header not found in config");
        assert_eq!(new_header.unwrap(), "header_value");
    }

    #[test]
    fn given_existing_header_when_set_header_then_should_overwrite_existing() {
        let mut config = HtrsConfigBuilder::new()
            .with_header("foo_header", "old_value")
            .build();
        let command = HeaderCommand::Set {
            header_name: "foo_header".to_string(),
            header_value: "new_value".to_string(),
        };

        let result = command.execute_command(&mut config);

        assert!(result.is_ok(), "Result was not ok: {}", result.err().unwrap());
        assert!(matches!(result.unwrap(), HtrsAction::UpdateConfig), "Returned action was not UpdateConfig");

        let header = config.get_header_value("foo_header".to_string());
        assert!(matches!(header, Some(_)), "Header not found in config");
        assert_eq!(header.unwrap(), "new_value");
    }

    #[test]
    fn given_existing_header_when_clear_header_then_should_be_removed() {
        let mut config = HtrsConfigBuilder::new()
            .with_header("header_name", "header_value")
            .build();
        let command = HeaderCommand::Clear {
            header_name: "header_name".to_string(),
        };

        let result = command.execute_command(&mut config);

        assert!(result.is_ok(), "Result was not ok: {}", result.err().unwrap());
        assert!(matches!(result.unwrap(), HtrsAction::UpdateConfig), "Returned action was not UpdateConfig");

        assert!(matches!(config.get_header_value("header_name".to_string()), None), "Header still exists in config");
    }
}

