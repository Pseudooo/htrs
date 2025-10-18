use crate::commands::delete_command::delete_environment_command::DeleteEnvironmentCommand;
use crate::commands::delete_command::DeleteCommand::{Environment, Service};
use crate::config::HtrsConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{ArgMatches, Command};
use delete_service_command::DeleteServiceCommand;

mod delete_service_command;
mod delete_environment_command;

pub enum DeleteCommand {
    Service(DeleteServiceCommand),
    Environment(DeleteEnvironmentCommand),
}

impl DeleteCommand {
    pub fn get_command() -> Command {
        Command::new("delete")
            .visible_alias("del")
            .about("Delete an existing item from config")
            .arg_required_else_help(true)
            .subcommand(DeleteServiceCommand::get_command())
            .subcommand(DeleteEnvironmentCommand::get_command())
    }

    pub fn bind_from_matches(args: &ArgMatches) -> DeleteCommand {
        match args.subcommand() {
            Some(("service", delete_service_matches)) => Service(DeleteServiceCommand::bind_from_matches(delete_service_matches)),
            Some(("environment" | "env", delete_environment_matches)) => Environment(DeleteEnvironmentCommand::bind_from_matches(delete_environment_matches)),
            _ => unreachable!(),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        match self {
            Service(delete_service_command) => delete_service_command.execute(config),
            Environment(delete_environment_command) => delete_environment_command.execute(config),
        }
    }
}
