use crate::command_args::RootCommands;
use crate::commands::call_command::CallServiceEndpointCommand;
use crate::commands::delete_command::DeleteCommand;
use crate::commands::edit_command::EditCommand;
use crate::commands::global_header_commands::GlobalHeaderCommand;
use crate::commands::list_command::ListCommand;
use crate::commands::new_command::NewCommand;
use crate::commands::service_commands::ServiceCommand;
use crate::commands::set_command::SetCommand;
use crate::config::HtrsConfig;
use crate::htrs_binding_error::HtrsBindingError;
use clap::{ArgMatches, Command};

pub trait MatchBinding<T> {
    fn bind_field(&self, field_id: &str) -> T;
}

impl MatchBinding<String> for ArgMatches {
    fn bind_field(&self, field_id: &str) -> String {
        let Some(field_value) = self.get_one::<String>(field_id) else {
            panic!("Unexpected binding - no value found");
        };
        field_value.clone()
    }
}

impl MatchBinding<Option<String>> for ArgMatches {
    fn bind_field(&self, field_id: &str) -> Option<String> {
        let Some(value) = self.get_one::<String>(field_id) else {
            return None
        };
        Some(value.clone())
    }
}

impl MatchBinding<bool> for ArgMatches {
    fn bind_field(&self, field_id: &str) -> bool {
        self.get_flag(field_id)
    }
}

impl MatchBinding<Option<bool>> for ArgMatches {
    fn bind_field(&self, field_id: &str) -> Option<bool> {
        let Some(value) = self.get_one::<bool>(field_id) else {
            return None;
        };

        Some(*value)
    }
}

impl MatchBinding<Vec<String>> for ArgMatches {
    fn bind_field(&self, field_id: &str) -> Vec<String> {
        let binding = self.get_many::<String>(field_id);
        let Some(binding_value) = binding else {
            return vec![];
        };
        binding_value.cloned().collect()
    }
}

impl RootCommands {
    pub fn bind_from_matches(config: &HtrsConfig, args: &ArgMatches) -> Result<RootCommands, HtrsBindingError> {
        match args.subcommand() {
            Some(("service", service_matches)) => {
                Ok(RootCommands::Service(
                    ServiceCommand::bind_from_matches(service_matches)
                ))
            },
            Some(("call", call_matches)) => {
                let call_service_endpoint_cmd = CallServiceEndpointCommand::bind_from_matches(config, call_matches)?;
                Ok(RootCommands::Call(
                    call_service_endpoint_cmd
                ))
            },
            Some(("header", header_matches)) => {
                Ok(RootCommands::Header(
                    GlobalHeaderCommand::bind_from_matches(header_matches)
                ))
            },
            Some(("new", new_matches)) => {
                Ok(RootCommands::New(
                    NewCommand::bind_from_matches(new_matches)
                ))
            },
            Some(("edit", edit_matches)) => {
                Ok(RootCommands::Edit(
                    EditCommand::bind_from_matches(edit_matches)
                ))
            },
            Some(("delete" | "del", delete_matches)) => {
                Ok(RootCommands::Delete(
                    DeleteCommand::bind_from_matches(delete_matches)
                ))
            }
            Some(("list" | "ls", list_matches)) => {
                Ok(RootCommands::List(
                    ListCommand::bind_from_matches(list_matches)
                ))
            }
            Some(("set", set_matches)) => {
                Ok(RootCommands::Set(
                    SetCommand::bind_from_matches(set_matches)
                ))
            }
            _ => panic!("Bad subcommand for RootCommands"),
        }
    }
}

pub fn get_root_command(config: &HtrsConfig) -> Command {
    Command::new("htrs")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A flexible http cli client")
        .arg_required_else_help(true)
        .subcommand(ServiceCommand::get_command())
        .subcommand(CallServiceEndpointCommand::get_command(config))
        .subcommand(GlobalHeaderCommand::get_command())
        .subcommand(NewCommand::get_command())
        .subcommand(EditCommand::get_command())
        .subcommand(DeleteCommand::get_command())
        .subcommand(ListCommand::get_command())
        .subcommand(SetCommand::get_command())
}
