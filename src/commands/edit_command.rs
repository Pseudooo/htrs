use crate::commands::edit_command::edit_endpoint_command::EditEndpointCommand;
use crate::commands::edit_command::edit_environment_command::EditEnvironmentCommand;
use crate::commands::edit_command::EditCommand::{Endpoint, Environment, Service};
use crate::config::HtrsConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{ArgMatches, Command};
use edit_service_command::EditServiceCommand;

mod edit_service_command;
mod edit_environment_command;
mod edit_endpoint_command;

pub enum EditCommand {
    Service(EditServiceCommand),
    Environment(EditEnvironmentCommand),
    Endpoint(EditEndpointCommand),
}

impl EditCommand {
    pub fn get_command() -> Command {
        Command::new("edit")
            .about("Edit an existing item in config")
            .arg_required_else_help(true)
            .subcommand(EditServiceCommand::get_command())
            .subcommand(EditEnvironmentCommand::get_command())
            .subcommand(EditEndpointCommand::get_command())
    }

    pub fn bind_from_matches(args: &ArgMatches) -> EditCommand {
        match args.subcommand() {
            Some(("service", service_matches)) => Service(EditServiceCommand::bind_from_matches(service_matches)),
            Some(("environment" | "env", environment_matches)) => Environment(EditEnvironmentCommand::bind_from_matches(environment_matches)),
            Some(("endpoint", endpoint_matches)) => Endpoint(EditEndpointCommand::bind_from_matches(endpoint_matches)),
            _ => unreachable!(),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        match self {
            Service(edit_service_command) => edit_service_command.execute(config),
            Environment(edit_environment_command) => edit_environment_command.execute(config),
            Endpoint(edit_endpoint_command) => edit_endpoint_command.execute(config),
        }
    }
}
