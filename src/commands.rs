pub mod call_command;
mod new_command;
mod edit_command;
mod delete_command;
mod list_command;
mod set_command;
mod bindings;

use crate::commands::call_command::CallServiceEndpointCommand;
use crate::commands::delete_command::DeleteCommand;
use crate::commands::edit_command::EditCommand;
use crate::commands::list_command::ListCommand;
use crate::commands::new_command::NewCommand;
use crate::commands::set_command::SetCommand;
use crate::commands::RootCommand::{Call, Delete, Edit, List, New, Set};
use crate::config::HtrsConfig;
use crate::htrs_binding_error::HtrsBindingError;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{ArgMatches, Command};

pub enum RootCommand {
    Call(CallServiceEndpointCommand),

    New(NewCommand),
    Edit(EditCommand),
    Delete(DeleteCommand),
    List(ListCommand),
    Set(SetCommand),
}

impl RootCommand {
    pub fn get_command(config: &HtrsConfig) -> Command {
        Command::new("htrs")
            .version(env!("CARGO_PKG_VERSION"))
            .about("A flexible http cli client")
            .arg_required_else_help(true)
            .subcommand(CallServiceEndpointCommand::get_command(config))
            .subcommand(NewCommand::get_command())
            .subcommand(EditCommand::get_command())
            .subcommand(DeleteCommand::get_command())
            .subcommand(ListCommand::get_command())
            .subcommand(SetCommand::get_command())
    }

    pub fn bind_from_matches(args: &ArgMatches, config: &HtrsConfig) -> Result<RootCommand, HtrsBindingError> {
        match args.subcommand() {
            Some(("call", call_matches)) => {
                let call_service_endpoint_cmd = CallServiceEndpointCommand::bind_from_matches(config, call_matches)?;
                Ok(Call(
                    call_service_endpoint_cmd
                ))
            },
            Some(("new", new_matches)) => {
                Ok(New(
                    NewCommand::bind_from_matches(new_matches)
                ))
            },
            Some(("edit", edit_matches)) => {
                Ok(Edit(
                    EditCommand::bind_from_matches(edit_matches)
                ))
            },
            Some(("delete" | "del", delete_matches)) => {
                Ok(Delete(
                    DeleteCommand::bind_from_matches(delete_matches)
                ))
            }
            Some(("list" | "ls", list_matches)) => {
                Ok(List(
                    ListCommand::bind_from_matches(list_matches)
                ))
            }
            Some(("set", set_matches)) => {
                Ok(Set(
                    SetCommand::bind_from_matches(set_matches)
                ))
            }
            _ => unreachable!()
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        match self {
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
}