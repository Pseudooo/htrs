use crate::commands::bindings::MatchBinding;
use crate::config::current_config::HtrsConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgMatches, Command};

pub struct ViewPresetCommand {
    pub name: String,
}

impl ViewPresetCommand {
    pub fn get_command() -> Command {
        Command::new("preset")
            .about("View a preset")
            .arg(
                Arg::new("name")
                    .help("Name or alias of the preset")
                    .value_name("name")
                    .required(true)
            )
    }

    pub fn bind_from_matches(args: &ArgMatches) -> ViewPresetCommand {
        ViewPresetCommand {
            name: args.bind_field("name")
        }
    }

    pub fn execute(&self, config: &HtrsConfig) -> Result<HtrsAction, HtrsError> {
        let Some(preset) = config.get_preset(self.name.as_str()) else {
            return Err(HtrsError::new(format!("No preset could be found with name or alias `{}`", self.name).as_str()));
        };

        let name = match preset.alias {
            Some(ref alias) => format!("{} ({}):", preset.name, alias),
            None => format!("{}:", preset.name),
        };

        let values = preset.values.iter()
            .map(|(key, value)| format!(" - {}: {}", key, value))
            .collect::<Vec<String>>()
            .join("\n");

        Ok(HtrsAction::PrintDialogue(format!("{}\n{}\n", name, values)))
    }
}