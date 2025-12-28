use crate::commands::bindings::MatchBinding;
use crate::config::current_config::HtrsConfig;
use crate::outcomes::HtrsAction::UpdateConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{value_parser, Arg, ArgMatches, Command};

pub struct EditEnvironmentCommand {
    pub name: String,
    pub service: String,
    pub new_name: Option<String>,
    pub new_alias: Option<String>,
    pub new_host: Option<String>,
    pub is_default: Option<bool>,
}

impl EditEnvironmentCommand {
    pub fn get_command() -> Command {
        Command::new("environment")
            .visible_alias("env")
            .about("Edit an existing environment in config")
            .arg(
                Arg::new("name")
                    .help("The current name or alias of for the existing environment")
                    .required(true)
            )
            .arg(
                Arg::new("service")
                    .help("The service that the targetted environment is defined under")
                    .required(true)
                    .long("service")
                    .short('s')
            )
            .arg(
                Arg::new("new-name")
                    .help("The new name for the environment")
                    .long("new-name")
                    .short('n')
                    .required(false)
            )
            .arg(
                Arg::new("new-alias")
                    .help("The new alias for the environment")
                    .long("new-alias")
                    .short('a')
                    .required(false)
            )
            .arg(
                Arg::new("new-host")
                    .help("The new host for the environment")
                    .long("new-host")
                    .required(false)
            )
            .arg(
                Arg::new("is-default")
                    .help("Should the environment be the default for the service")
                    .long("is-default")
                    .short('d')
                    .required(false)
                    .value_parser(value_parser!(bool))
            )
    }

    pub fn bind_from_matches(args: &ArgMatches) -> EditEnvironmentCommand {
        EditEnvironmentCommand {
            name: args.bind_field("name"),
            service: args.bind_field("service"),
            new_name: args.bind_field("new-name"),
            new_alias: args.bind_field("new-alias"),
            new_host: args.bind_field("new-host"),
            is_default: args.bind_field("is-default"),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        let Some(service) = config.get_service_mut(&self.service) else {
            return Err(HtrsError::new(format!("No service could be found with name or alias `{}`", self.service).as_ref()))
        };
        if service.get_environment_mut(&self.name).is_none() {
            return Err(HtrsError::new(format!("No environment could be found with name or alias `{}`", self.name).as_ref()))
        };

        if self.new_name.is_some() && service.get_environment_mut(self.new_name.as_ref().unwrap()).is_some() {
            return Err(HtrsError::new(format!("Service `{}` already has an environment with name or alias `{}`", service.name, self.new_name.as_ref().unwrap()).as_ref()))
        }
        if self.new_alias.is_some() && service.get_environment_mut(self.new_alias.as_ref().unwrap()).is_some() {
            return Err(HtrsError::new(format!("Service `{}` already has an environment with name or alias `{}`", service.name, self.new_alias.as_ref().unwrap()).as_ref()))
        }

        if self.is_default.unwrap_or(false) {
            if let Some(existing_default_environment) = service.get_default_environment_mut() {
                existing_default_environment.default = false;
            }
        }

        let environment = service.get_environment_mut(&self.name).unwrap();

        if let Some(new_name) = &self.new_name {
            environment.name = new_name.clone();
        }
        if let Some(new_alias) = &self.new_alias {
            environment.alias = Some(new_alias.clone());
        }
        if let Some(new_host) = &self.new_host {
            environment.host = new_host.clone();
        }
        if let Some(is_default) = &self.is_default {
            environment.default = *is_default;
        }
        Ok(UpdateConfig)
    }
}
