use crate::commands::delete_command::DeleteCommand::Service;
use crate::config::HtrsConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{ArgMatches, Command};
use delete_service_command::DeleteServiceCommand;

mod delete_service_command;

pub enum DeleteCommand {
    Service(DeleteServiceCommand),
}

impl DeleteCommand {
    pub fn get_command() -> Command {
        Command::new("delete")
            .visible_alias("del")
            .about("Delete an existing item from config")
            .arg_required_else_help(true)
            .subcommand(DeleteServiceCommand::get_command())
    }

    pub fn bind_from_matches(args: &ArgMatches) -> DeleteCommand {
        match args.subcommand() {
            Some(("service", delete_service_matches)) => Service(DeleteServiceCommand::bind_from_matches(delete_service_matches)),
            _ => unreachable!(),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        match self {
            Service(delete_service_command) => delete_service_command.execute(config),
        }
    }
}
