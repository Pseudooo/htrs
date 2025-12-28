use crate::commands::bindings::MatchBinding;
use crate::config::current_config::{HtrsConfig, Service};
use crate::outcomes::HtrsAction::UpdateConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgMatches, Command};

pub struct NewServiceCommand {
    pub name: String,
    pub alias: Option<String>,
}

impl NewServiceCommand {
    pub fn get_command() -> Command {
        Command::new("service")
            .about("Create a new service")
            .arg_required_else_help(true)
            .arg(
                Arg::new("name")
                    .help("The unique name of the service to create")
                    .required(true)
            )
            .arg(
                Arg::new("alias")
                    .help("The unique alias for the new service")
                    .long("alias")
                    .short('a')
                    .required(false)
            )
    }

    pub fn bind_from_matches(args: &ArgMatches) -> NewServiceCommand {
        NewServiceCommand {
            name: args.bind_field("name"),
            alias: args.bind_field("alias"),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        if config.get_service(&self.name).is_some() {
            return Err(HtrsError::new(format!("A service already exists with the name or alias '{}'", self.name).as_str()));
        }
        if let Some(alias) = &self.alias {
            if config.get_service(alias).is_some() {
                return Err(HtrsError::new(format!("A service already exists with the name or alias '{alias}").as_str()));
            }
        }

        config.services.push(Service::new(self.name.clone(), self.alias.clone()));
        Ok(UpdateConfig)
    }
}
