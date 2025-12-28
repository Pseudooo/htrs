use crate::commands::bindings::MatchBinding;
use crate::config::current_config::HtrsConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgMatches, Command};

pub struct DeleteServiceCommand {
    pub name: String,
}

impl DeleteServiceCommand {
    pub fn get_command() -> Command {
        Command::new("service")
            .about("Delete an existing service from config")
            .arg(
                Arg::new("name")
                    .help("The name or alias of the service to delete")
                    .required(true)
            )
    }

    pub fn bind_from_matches(args: &ArgMatches) -> DeleteServiceCommand {
        DeleteServiceCommand {
            name: args.bind_field("name"),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        match config.remove_service(&self.name) {
            true => Ok(HtrsAction::UpdateConfig),
            false => Err(HtrsError::aliased_item_not_found("service", self.name.as_str())),
        }
    }
}
