pub mod call_command;
pub mod service_commands;
pub mod global_header_commands;
mod environment_commands;
mod endpoint_commands;
mod service_header_commands;
pub(crate) mod new_command;
pub(crate) mod edit_command;
mod new_service_command;
mod edit_service_command;

use crate::command_args::RootCommands;
use crate::command_args::RootCommands::{Call, Edit, Header, New, Service};
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
        New(new_command) => new_command.execute(config),
        Edit(edit_command) => edit_command.execute(config),
    }
}