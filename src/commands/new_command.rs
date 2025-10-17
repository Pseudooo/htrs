use crate::commands::new_command::NewCommand::Service;
use crate::commands::new_service_command::CreateNewServiceCommand;
use crate::config::HtrsConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{ArgMatches, Command};

pub enum NewCommand {
    Service(CreateNewServiceCommand),
}

impl NewCommand {
    pub fn get_command() -> Command {
        Command::new("new")
            .about("Create a new item in config")
            .arg_required_else_help(true)
            .subcommand(CreateNewServiceCommand::get_command())
    }

    pub fn bind_from_matches(args: &ArgMatches) -> NewCommand {
        match args.subcommand() {
            Some(("service", service_matches)) => Service(CreateNewServiceCommand::bind_from_matches(service_matches)),
            _ => unreachable!(),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        match self {
            Service(create_new_service_command) => create_new_service_command.execute(config),
        }
    }
}
