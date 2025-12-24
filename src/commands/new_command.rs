mod new_environment_command;
mod new_service_command;
mod new_endpoint_command;
mod new_preset_command;

use crate::commands::new_command::new_endpoint_command::NewEndpointCommand;
use crate::commands::new_command::new_environment_command::NewEnvironmentCommand;
use crate::commands::new_command::new_preset_command::NewPresetCommand;
use crate::commands::new_command::NewCommand::{Endpoint, Environment, Preset, Service};
use crate::config::HtrsConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{ArgMatches, Command};
use new_service_command::NewServiceCommand;

pub enum NewCommand {
    Service(NewServiceCommand),
    Environment(NewEnvironmentCommand),
    Endpoint(NewEndpointCommand),
    Preset(NewPresetCommand),
}

impl NewCommand {
    pub fn get_command() -> Command {
        Command::new("new")
            .about("Create a new item in config")
            .arg_required_else_help(true)
            .subcommand(NewServiceCommand::get_command())
            .subcommand(NewEnvironmentCommand::get_command())
            .subcommand(NewEndpointCommand::get_command())
            .subcommand(NewPresetCommand::get_command())
    }

    pub fn bind_from_matches(args: &ArgMatches) -> NewCommand {
        match args.subcommand() {
            Some(("service", service_matches)) => Service(NewServiceCommand::bind_from_matches(service_matches)),
            Some(("environment" | "env", environment_matches)) => Environment(NewEnvironmentCommand::bind_from_matches(environment_matches)),
            Some(("endpoint", endpoint_matches)) => Endpoint(NewEndpointCommand::bind_from_matches(endpoint_matches)),
            Some(("preset", preset_matches)) => Preset(NewPresetCommand::bind_from_matches(preset_matches)),
            _ => unreachable!(),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        match self {
            Service(create_new_service_command) => create_new_service_command.execute(config),
            Environment(create_new_environment_command) => create_new_environment_command.execute(config),
            Endpoint(create_new_endpoint_command) => create_new_endpoint_command.execute(config),
            Preset(create_new_preset_command) => create_new_preset_command.execute(config),
        }
    }
}
