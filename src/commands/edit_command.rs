use crate::commands::edit_command::EditCommand::Service;
use crate::config::HtrsConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{ArgMatches, Command};
use edit_service_command::EditServiceCommand;

mod edit_service_command;

pub enum EditCommand {
    Service(EditServiceCommand),
}

impl EditCommand {
    pub fn get_command() -> Command {
        Command::new("edit")
            .about("Edit an existing item in config")
            .arg_required_else_help(true)
            .subcommand(EditServiceCommand::get_command())
    }

    pub fn bind_from_matches(args: &ArgMatches) -> EditCommand {
        match args.subcommand() {
            Some(("service", service_matches)) => Service(EditServiceCommand::bind_from_matches(service_matches)),
            _ => unreachable!(),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        match self {
            Service(edit_service_command) => edit_service_command.execute(config),
        }
    }
}
