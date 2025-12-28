use crate::commands::bindings::MatchBinding;
use crate::config::current_config::HtrsConfig;
use crate::outcomes::HtrsAction::UpdateConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgMatches, Command};

pub struct DeletePresetCommand {
    pub name: String,
}

impl DeletePresetCommand {
    pub fn get_command() -> Command {
        Command::new("preset")
            .about("Delete an existing preset")
            .arg_required_else_help(true)
            .arg(
                Arg::new("name")
                    .help("The name of the preset to delete")
                    .required(true)
            )
    }

    pub fn bind_from_matches(args: &ArgMatches) -> DeletePresetCommand {
        DeletePresetCommand {
            name: args.bind_field("name"),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        match config.remove_preset(self.name.as_str()) {
            true => Ok(UpdateConfig),
            false => Err(HtrsError::new(format!("Unable to find preset with name `{}`", self.name).as_str())),
        }
    }
}