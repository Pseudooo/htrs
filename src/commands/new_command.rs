mod new_environment_command;
mod new_service_command;

use crate::commands::new_command::NewCommand::Service;
use crate::config::HtrsConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{ArgMatches, Command};
use new_service_command::NewServiceCommand;

pub enum NewCommand {
    Service(NewServiceCommand),
}

impl NewCommand {
    pub fn get_command() -> Command {
        Command::new("new")
            .about("Create a new item in config")
            .arg_required_else_help(true)
            .subcommand(NewServiceCommand::get_command())
    }

    pub fn bind_from_matches(args: &ArgMatches) -> NewCommand {
        match args.subcommand() {
            Some(("service", service_matches)) => Service(NewServiceCommand::bind_from_matches(service_matches)),
            _ => unreachable!(),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        match self {
            Service(create_new_service_command) => create_new_service_command.execute(config),
        }
    }
}
