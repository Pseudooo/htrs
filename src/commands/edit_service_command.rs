use crate::command_builder::MatchBinding;
use crate::config::HtrsConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgMatches, Command};

pub struct EditServiceCommand {
    pub name: String,
    pub new_name: Option<String>,
    pub new_alias: Option<String>,
}

impl EditServiceCommand {
    pub fn get_command() -> Command {
        Command::new("service")
            .about("Edit an existing service")
            .arg(
                Arg::new("name")
                    .help("The current name or alias for the service to edit")
                    .required(true)
            )
            .arg(
                Arg::new("new-name")
                    .help("The new name for the service")
                    .long("new-name")
                    .short('n')
                    .required(false)
            )
            .arg(
                Arg::new("new-alias")
                    .help("The new alias for the service")
                    .long("new-alias")
                    .short('a')
                    .required(false)
            )
    }

    pub fn bind_from_matches(args: &ArgMatches) -> EditServiceCommand {
        EditServiceCommand {
            name: args.bind_field("name"),
            new_name: args.bind_field("new-name"),
            new_alias: args.bind_field("new-alias"),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        if self.new_name.is_some() && config.get_service(self.new_name.as_ref().unwrap()).is_some() {
            return Err(HtrsError::new(format!("A service already exists with the name or alias `{}`", &self.new_name.as_ref().unwrap()).as_str()))
        }
        if self.new_alias.is_some() && config.get_service(self.new_alias.as_ref().unwrap()).is_some() {
            return Err(HtrsError::new(format!("A service already exists with the name or alias `{}`", &self.new_alias.as_ref().unwrap()).as_str()))
        }

        let Some(service) = &mut config.get_service_mut(&self.name) else {
            return Err(HtrsError::new(format!("No service found with name or alias `{}`", &self.name).as_str()))
        };

        if self.new_name.is_some() {
            service.name = self.new_name.clone().unwrap();
        }
        if self.new_alias.is_some() {
            service.alias = self.new_alias.clone();
        }
        Ok(HtrsAction::UpdateConfig)
    }
}
