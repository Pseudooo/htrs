use crate::commands::bindings::MatchBinding;
use crate::config::current_config::{Environment, HtrsConfig};
use crate::outcomes::HtrsAction::UpdateConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgMatches, Command};

pub struct NewEnvironmentCommand {
    pub service: String,
    pub name: String,
    pub alias: Option<String>,
    pub host: String,
    pub default: bool,
}

impl NewEnvironmentCommand {
    pub fn get_command() -> Command {
        Command::new("environment")
            .visible_alias("env")
            .about("Create a new environment")
            .arg(
                Arg::new("name")
                    .help("The unique name for the new environment")
                    .required(true)
            )
            .arg(
                Arg::new("host")
                    .help("The host for the environment")
                    .required(true)
            )
            .arg(
                Arg::new("default")
                    .help("Flag to determine if the new environment should be the default")
                    .required(false)
                    .num_args(0)
                    .long("default")
            )
            .arg(
                Arg::new("alias")
                    .help("The unique alias for the new environment")
                    .required(false)
                    .long("alias")
                    .short('a')
            )
            .arg(
                Arg::new("service")
                    .help("The service that the environment will be created for")
                    .required(true)
                    .long("service")
                    .short('s')
            )
    }

    pub fn bind_from_matches(args: &ArgMatches) -> NewEnvironmentCommand {
        NewEnvironmentCommand {
            service: args.bind_field("service"),
            name: args.bind_field("name"),
            alias: args.bind_field("alias"),
            host: args.bind_field("host"),
            default: args.bind_field("default"),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        let Some(service) = config.get_service_mut(&self.service) else {
            return Err(HtrsError::new(format!("No service found with name or alias `{}`", self.service).as_str()));
        };

        if service.get_environment(&self.name).is_some() {
            return Err(HtrsError::new(format!("Service `{}` already has an environment with name or alias `{}`", service.name, self.name).as_str()));
        }
        if self.alias.is_some() && service.get_environment(self.alias.as_ref().unwrap().as_str()).is_some() {
            return Err(HtrsError::new(format!("Service `{}` already has an environment with name or alias `{}`", service.name, self.alias.as_ref().unwrap()).as_str()));
        }

        if self.default {
            if let Some(existing_default_environment) = service.get_default_environment_mut() {
                existing_default_environment.default = false;
            }
        }

        service.environments.push(
            Environment::new(
                self.name.clone(),
                self.alias.clone(),
                self.host.clone(),
                self.default)
        );
        Ok(UpdateConfig)
    }
}
