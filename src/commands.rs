pub(crate) mod call_command;
pub(crate) mod service_commands;
pub(crate) mod header_commands;
mod environment_commands;
mod endpoint_commands;

use crate::command_args::RootCommands;
use crate::command_args::RootCommands::{Call, Header, Service};
use crate::config::HtrsConfig;
use crate::outcomes::{HtrsAction, HtrsError};

pub fn execute_command(config: &mut HtrsConfig, cmd: RootCommands) -> Result<HtrsAction, HtrsError> {
    match cmd {
        Service(service_command) => {
            service_command.execute_command(config)
        },
        Call(call_command) => {
            call_command.execute_command(config)
        },
        Header(header_command) => {
            header_command.execute_command(config)
        },
    }
}