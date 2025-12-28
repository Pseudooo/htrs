use crate::commands::bindings::MatchBinding;
use crate::common::parse_key_value_string;
use crate::config::current_config::{HtrsConfig, Preset};
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgAction, ArgMatches, Command};
use std::collections::HashMap;

pub struct NewPresetCommand {
    pub name: String,
    pub alias: Option<String>,
    pub values: Vec<String>,
}

impl NewPresetCommand {
    pub fn get_command() -> Command {
        Command::new("preset")
            .about("Create a new preset")
            .arg_required_else_help(true)
            .arg(
                Arg::new("name")
                    .help("The preset name")
                    .required(true)
            )
            .arg(
                Arg::new("alias")
                    .help("Alias for the preset")
                    .required(false)
                    .long("alias")
                    .short('a')
            )
            .arg(
                Arg::new("value")
                    .help("A parameter value to be included in the preset, should be given in format <key>=<value>")
                    .long("value")
                    .short('v')
                    .required(true)
                    .action(ArgAction::Append)
            )
    }

    pub fn bind_from_matches(args: &ArgMatches) -> NewPresetCommand {
        NewPresetCommand {
            name: args.bind_field("name"),
            alias: args.bind_field("alias"),
            values: args.bind_field("value"),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        if config.get_preset(self.name.as_str()).is_some() {
            return Err(HtrsError::new(format!("A preset with name or alias `{}` already exists", self.name).as_str()));
        }
        if self.alias.is_some() && config.get_preset(self.alias.as_ref().unwrap()).is_some() {
            return Err(HtrsError::new(format!("A preset with name or alias `{}` already exists", self.alias.as_ref().unwrap()).as_str()));
        }

        let mut values = HashMap::new();
        for value in &self.values {
            if let Ok((left, right)) = parse_key_value_string(value) {
                values.insert(left, right);
            } else {
                return Err(HtrsError::new(format!("Invalid preset value `{}`, should be in format `key=value`", value).as_str()));
            }
        }

        config.presets.push(Preset {
            name: self.name.to_string(),
            alias: self.alias.clone(),
            values,
        });
        Ok(HtrsAction::UpdateConfig)
    }
}
