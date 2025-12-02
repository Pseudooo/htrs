use crate::commands::delete_command::delete_endpoint_command::DeleteEndpointCommand;
use crate::commands::delete_command::delete_environment_command::DeleteEnvironmentCommand;
use crate::commands::delete_command::delete_header_command::DeleteHeaderCommand;
use crate::commands::delete_command::delete_preset_command::DeletePresetCommand;
use crate::commands::delete_command::DeleteCommand::{Endpoint, Environment, Header, Preset, Service};
use crate::config::HtrsConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{ArgMatches, Command};
use delete_service_command::DeleteServiceCommand;

mod delete_service_command;
mod delete_environment_command;
mod delete_header_command;
mod delete_endpoint_command;
mod delete_preset_command;

pub enum DeleteCommand {
    Service(DeleteServiceCommand),
    Environment(DeleteEnvironmentCommand),
    Header(DeleteHeaderCommand),
    Endpoint(DeleteEndpointCommand),
    Preset(DeletePresetCommand),
}

impl DeleteCommand {
    pub fn get_command() -> Command {
        Command::new("delete")
            .visible_alias("del")
            .about("Delete an existing item from config")
            .arg_required_else_help(true)
            .subcommand(DeleteServiceCommand::get_command())
            .subcommand(DeleteEnvironmentCommand::get_command())
            .subcommand(DeleteHeaderCommand::get_command())
            .subcommand(DeleteEndpointCommand::get_command())
            .subcommand(DeletePresetCommand::get_command())
    }

    pub fn bind_from_matches(args: &ArgMatches) -> DeleteCommand {
        match args.subcommand() {
            Some(("service", delete_service_matches)) => Service(DeleteServiceCommand::bind_from_matches(delete_service_matches)),
            Some(("environment" | "env", delete_environment_matches)) => Environment(DeleteEnvironmentCommand::bind_from_matches(delete_environment_matches)),
            Some(("header", delete_header_matches)) => Header(DeleteHeaderCommand::bind_from_matches(delete_header_matches)),
            Some(("endpoint", delete_endpoint_matches)) => Endpoint(DeleteEndpointCommand::bind_from_matches(delete_endpoint_matches)),
            Some(("preset", delete_preset_matches)) => Preset(DeletePresetCommand::bind_from_matches(delete_preset_matches)),
            _ => unreachable!(),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        match self {
            Service(delete_service_command) => delete_service_command.execute(config),
            Environment(delete_environment_command) => delete_environment_command.execute(config),
            Header(delete_header_command) => delete_header_command.execute(config),
            Endpoint(delete_endpoint_command) => delete_endpoint_command.execute(config),
            Preset(delete_preset_command) => delete_preset_command.execute(config),
        }
    }
}
