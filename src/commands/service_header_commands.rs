use crate::command_builder::MatchBinding;
use crate::config::{HtrsConfig, Service};
use crate::outcomes::HtrsAction::UpdateConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgMatches, Command};

pub enum ServiceHeaderCommand {
    Set {
        service: String,
        header_name: String,
        header_value: String,
    },
    Clear {
        service: String,
        header_name: String,
    }
}

impl ServiceHeaderCommand {
    pub fn get_command() -> Command {
        Command::new("header")
            .about("Set a header for a service")
            .arg_required_else_help(true)
            .arg(
                Arg::new("service")
                    .help("Service name or alias")
                    .value_name("service")
                    .required(true)
            )
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

    pub fn bind_from_matches(args: &ArgMatches) -> ServiceHeaderCommand {
        let service = args.bind_field("service");
        match args.subcommand() {
            Some(("set", set_matches)) => {
                ServiceHeaderCommand::Set {
                    service,
                    header_name: set_matches.bind_field("header_name"),
                    header_value: set_matches.bind_field("header_value"),
                }
            },
            Some(("clear", clear_matches)) => {
                ServiceHeaderCommand::Clear {
                    service,
                    header_name: clear_matches.bind_field("header_name"),
                }
            },
            _ => panic!("Bad subcommand for ServiceHeaderCommand")
        }
    }

    pub fn execute_command(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        match self {
            ServiceHeaderCommand::Set {
                service, header_name, header_value
            } => {
                let service = get_service_or_err(config, service.clone())?;
                service.set_header(header_name.clone(), header_value.clone());
                Ok(UpdateConfig)
            }
            ServiceHeaderCommand::Clear { service, header_name } => {
                let service = get_service_or_err(config, service.clone())?;
                service.clear_header(header_name.clone());
                Ok(UpdateConfig)
            }
        }
    }
}

fn get_service_or_err(config: &mut HtrsConfig, service_name: String) -> Result<&mut Service, HtrsError> {
    let Some(service) = config.get_service_mut(service_name.as_str()) else {
        return Err(HtrsError::new(format!("Service '{service_name}` not found").as_str()));
    };

    Ok(service)
}

#[cfg(test)]
mod service_header_command_binding_tests {
    use super::*;
    use clap::Error;

    fn get_parsed_command(args: Vec<&str>) -> Result<ServiceHeaderCommand, Error> {
        let command = ServiceHeaderCommand::get_command();
        let matches = command.try_get_matches_from(args)?;
        Ok(ServiceHeaderCommand::bind_from_matches(&matches))
    }

    #[test]
    fn given_valid_set_header_command_then_should_parse_and_map() {
        let args = vec!["htrs", "foo_service", "set", "header_name", "header_value"];

        let result = get_parsed_command(args);

        assert!(result.is_ok(), "Result was not ok: {}", result.err().unwrap());
        let ServiceHeaderCommand::Set {
            service,
            header_name,
            header_value
        } = result.unwrap() else {
            panic!("Command was not ServiceHeaderCommand::Set");
        };
        assert_eq!(service, "foo_service");
        assert_eq!(header_name, "header_name");
        assert_eq!(header_value, "header_value");
    }

    #[test]
    fn given_valid_clear_header_command_then_should_parse_and_map() {
        let args = vec!["htrs", "foo_service", "clear", "header_name"];

        let result = get_parsed_command(args);

        assert!(result.is_ok(), "Result was not ok: {}", result.err().unwrap());
        let ServiceHeaderCommand::Clear {
            service,
            header_name,
        } = result.unwrap() else {
            panic!("Command was not ServiceHeaderCommand::Clear");
        };
        assert_eq!(service, "foo_service");
        assert_eq!(header_name, "header_name");
    }
}
