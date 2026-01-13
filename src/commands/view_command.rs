use crate::commands::view_command::view_preset_command::ViewPresetCommand;
use crate::commands::view_command::view_service_command::ViewServiceCommand;
use crate::commands::view_command::ViewCommand::{Preset, Service};
use crate::config::current_config::HtrsConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{ArgMatches, Command};

mod view_preset_command;
mod view_service_command;

pub enum ViewCommand {
    Preset(ViewPresetCommand),
    Service(ViewServiceCommand),
}

impl ViewCommand {
    pub fn get_command() -> Command {
        Command::new("view")
            .about("View an item")
            .arg_required_else_help(true)
            .subcommand(ViewPresetCommand::get_command())
            .subcommand(ViewServiceCommand::get_command())
    }

    pub fn bind_from_matches(args: &ArgMatches) -> ViewCommand {
        match args.subcommand() {
            Some(("preset", preset_matches)) => Preset(ViewPresetCommand::bind_from_matches(preset_matches)),
            Some(("service", service_matches)) => Service(ViewServiceCommand::bind_from_matches(service_matches)),
            _ => unreachable!(),
        }
    }

    pub fn execute(&self, config: &HtrsConfig) -> Result<HtrsAction, HtrsError> {
        match self {
            Preset(command) => command.execute(config),
            Service(command) => command.execute(config),
        }
    }
}
