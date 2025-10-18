use crate::commands::list_command::ListCommand::Service;
use crate::config::HtrsConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{ArgMatches, Command};
use list_service_command::ListServicesCommand;

mod list_service_command;

pub enum ListCommand {
    Service(ListServicesCommand),
}

impl ListCommand {
    pub fn get_command() -> Command {
        Command::new("list")
            .about("List items defined in config")
            .arg_required_else_help(true)
            .subcommand(ListServicesCommand::get_command())
    }

    pub fn bind_from_matches(args: &ArgMatches) -> ListCommand {
        match args.subcommand() {
            Some(("service", service_matches)) => Service(ListServicesCommand::bind_from_matches(service_matches)),
            _ => unreachable!(),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        match self {
            Service(list_services_command) => list_services_command.execute(config),
        }
    }
}
