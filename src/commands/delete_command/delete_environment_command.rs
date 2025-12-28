use crate::commands::bindings::MatchBinding;
use crate::config::current_config::HtrsConfig;
use crate::outcomes::HtrsAction::UpdateConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgMatches, Command};

pub struct DeleteEnvironmentCommand {
    pub name: String,
    pub service: String,
}

impl DeleteEnvironmentCommand {
    pub fn get_command() -> Command {
        Command::new("environment")
            .about("Delete an existing environment from config")
            .visible_alias("env")
            .arg(
                Arg::new("name")
                    .help("The name or alias of the environment to delete")
                    .required(true)
            )
            .arg(
                Arg::new("service")
                    .help("The service name or alias that environment is defined in")
                    .required(true)
                    .long("service")
                    .short('s')
            )
    }

    pub fn bind_from_matches(args: &ArgMatches) -> DeleteEnvironmentCommand {
        DeleteEnvironmentCommand {
            name: args.bind_field("name"),
            service: args.bind_field("service"),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        let Some(service) = config.get_service_mut(&self.service) else {
            return Err(HtrsError::new(format!("No service could be found with name or alias `{}`", self.service).as_str()))
        };
        match service.remove_environment(&self.name) {
            true => Ok(UpdateConfig),
            false => Err(HtrsError::new(format!("No environment could be found with name or alias `{}`", self.name).as_str()))
        }
    }
}
