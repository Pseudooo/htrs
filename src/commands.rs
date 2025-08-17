mod service_commands;
pub(crate) mod call_commands;

use crate::command_args::ConfigurationCommands::Header;
use crate::command_args::HeaderCommands::{Clear, Set};
use crate::command_args::RootCommands;
use crate::command_args::RootCommands::{Call, Service};
use crate::commands::call_commands::execute_call_command;
use crate::commands::service_commands::execute_service_command;
use crate::config::HtrsConfig;
use crate::outcomes::HtrsAction::UpdateConfig;
use crate::outcomes::{HtrsAction, HtrsError};

impl RootCommands {

}

pub fn execute_command(config: &mut HtrsConfig, cmd: RootCommands) -> Result<HtrsAction, HtrsError> {
    match cmd {
        Service(service_command) => {
            execute_service_command(config, &service_command)
        },
        Call(options) => {
            execute_call_command(config, options)
        },
        RootCommands::Config(config_cmd) => {
            let Header(header_cmd) = config_cmd;
            match header_cmd {
                Set { header, value } => {
                    config.headers.insert(header, value);
                    Ok(UpdateConfig)
                },
                Clear { header } => {
                    if config.headers.remove(&header) == None {
                        Err(HtrsError::new(&format!("No header `{}` defined", header)))
                    } else {
                        Ok(UpdateConfig)
                    }
                },
            }
        },
    }
}