use crate::commands::set_command::set_header_command::SetHeaderCommand;
use crate::commands::set_command::SetCommand::Header;
use crate::config::current_config::HtrsConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{ArgMatches, Command};

mod set_header_command;

pub enum SetCommand {
    Header(SetHeaderCommand),
}

impl SetCommand {
    pub fn get_command() -> Command {
        Command::new("set")
            .about("Set a value for an item in config")
            .arg_required_else_help(true)
            .subcommand(SetHeaderCommand::get_command())
    }

    pub fn bind_from_matches(args: &ArgMatches) -> SetCommand {
        match args.subcommand() {
            Some(("header", header_matches)) => Header(SetHeaderCommand::bind_from_matches(header_matches)),
            _ => unreachable!(),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        match self {
            Header(header) => header.execute(config),
        }
    }
}
