use crate::commands::bindings::MatchBinding;
use crate::common::parse_key_value_string;
use crate::config::current_config::HtrsConfig;
use crate::htrs_binding_error::HtrsBindingError;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgAction, ArgMatches, Command};

pub struct EditPresetCommand {
    pub name: String,
    pub new_name: Option<String>,
    pub new_alias: Option<String>,
    pub set_values: Vec<(String, String)>,
    pub clear_values: Vec<String>,
}

impl EditPresetCommand {
    pub fn get_command() -> Command {
        Command::new("preset")
            .arg(
                Arg::new("name")
                    .help("The name of the preset to edit")
                    .required(true)
            )
            .arg(
                Arg::new("new-name")
                    .help("New name of the preset")
                    .required(false)
                    .long("new-name")
            )
            .arg(
                Arg::new("new-alias")
                    .help("The new alias of the preset")
                    .required(false)
                    .long("new-alias")
            )
            .arg(
                Arg::new("set")
                    .help("Set a parameter value in the format `key=value`")
                    .required(false)
                    .action(ArgAction::Append)
                    .long("set")
                    .short('s')
            )
            .arg(
                Arg::new("clear")
                    .help("Clear an existing parameter value by name")
                    .required(false)
                    .action(ArgAction::Append)
                    .long("clear")
                    .short('c')
            )
    }

    pub fn bind_from_matches(args: &ArgMatches) -> Result<EditPresetCommand, HtrsBindingError> {
        let unparsed_set_values: Vec<String> = args.bind_field("set");
        let mut set_values: Vec<(String, String)> = vec![];
        for value in unparsed_set_values {
            match parse_key_value_string(value.as_str()) {
                Ok((key, value)) => set_values.push((key, value)),
                Err(_) => return Err(HtrsBindingError {
                    description: format!("Invalid set value `{}`, should be in format `key=value`", value)
                }),
            };
        }

        Ok(EditPresetCommand {
            name: args.bind_field("name"),
            new_name: args.bind_field("new-name"),
            new_alias: args.bind_field("new-alias"),
            set_values,
            clear_values: args.bind_field("clear"),
        })
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        if self.new_name.is_some() && config.get_preset(self.new_name.as_ref().unwrap()).is_some() {
            return Err(HtrsError::new(format!("A preset already exists with name `{}`", self.new_name.as_ref().unwrap()).as_str()));
        }
        if self.new_alias.is_some() && config.get_preset(self.new_alias.as_ref().unwrap()).is_some() {
            return Err(HtrsError::new(format!("A preset already exists with name or alias `{}`", self.new_alias.as_ref().unwrap()).as_str()));
        }

        let Some(preset) = config.get_preset_mut(self.name.as_str()) else {
            return Err(HtrsError::new(format!("No preset found with name `{}`", self.name).as_str()));
        };

        if let Some(new_name) = &self.new_name {
            preset.name = new_name.clone();
        }
        for (key, value) in &self.set_values {
            preset.values.insert(key.clone(), value.clone());
        }
        for key in &self.clear_values {
            if preset.values.contains_key(key) {
                preset.values.remove(key.as_str());
            } else {
                return Err(HtrsError::new(format!("Preset `{}` has no parameter `{}`", preset.name, key).as_str()));
            }
        }

        Ok(HtrsAction::UpdateConfig)
    }
}
