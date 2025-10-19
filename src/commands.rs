pub mod call_command;
pub mod service_commands;
mod endpoint_commands;
pub(crate) mod new_command;
pub(crate) mod edit_command;
pub(crate) mod delete_command;
pub(crate) mod list_command;
pub(crate) mod set_command;

use crate::command_args::RootCommands;
use crate::command_args::RootCommands::{Call, Delete, Edit, List, New, Service, Set};
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
        New(new_command) => new_command.execute(config),
        Edit(edit_command) => edit_command.execute(config),
        Delete(delete_command) => delete_command.execute(config),
        List(list_command) => list_command.execute(config),
        Set(set_command) => set_command.execute(config),
    }
}