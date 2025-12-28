use crate::commands::bindings::MatchBinding;
use crate::config::current_config::HtrsConfig;
use crate::outcomes::HtrsAction::UpdateConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgMatches, Command};

pub struct DeleteEndpointCommand {
    pub name: String,
    pub service: String,
}

impl DeleteEndpointCommand {
    pub fn get_command() -> Command {
        Command::new("endpoint")
            .about("Delete an existing endpoint from config")
            .arg(
                Arg::new("name")
                    .help("The name of the endpoint")
                    .required(true)
            )
            .arg(
                Arg::new("service")
                    .help("The service name or alias that the endpoint is defined for")
                    .required(true)
                    .long("service")
                    .short('s')
            )
    }

    pub fn bind_from_matches(args: &ArgMatches) -> DeleteEndpointCommand {
        DeleteEndpointCommand {
            name: args.bind_field("name"),
            service: args.bind_field("service"),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        let Some(service) = config.get_service_mut(&self.service) else {
            return Err(HtrsError::aliased_item_not_found("service", self.service.as_str()));
        };
        match service.remove_endpoint(&self.name) {
            true => Ok(UpdateConfig),
            false => Err(HtrsError::item_not_found("endpoint", self.name.as_str()))
        }
    }
}
