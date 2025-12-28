use crate::commands::bindings::MatchBinding;
use crate::config::current_config::HtrsConfig;
use crate::config::HeaderItem;
use crate::outcomes::HtrsAction::UpdateConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgMatches, Command};

pub struct DeleteHeaderCommand {
    pub header_name: String,
    pub service: Option<String>,
    pub environment: Option<String>,
}

impl DeleteHeaderCommand {
    pub fn get_command() -> Command {
        Command::new("header")
            .about("Delete a header that's been set in config")
            .arg(
                Arg::new("name")
                    .help("The header name to remove")
                    .required(true)
            )
            .arg(
                Arg::new("service")
                    .help("The service to target")
                    .required(false)
                    .long("service")
                    .short('s')
            )
            .arg(
                Arg::new("environment")
                    .help("The environment to target")
                    .required(false)
                    .long("environment")
                    .short('e')
            )
    }

    pub fn bind_from_matches(args: &ArgMatches) -> DeleteHeaderCommand {
        DeleteHeaderCommand {
            header_name: args.bind_field("name"),
            service: args.bind_field("service"),
            environment: args.bind_field("environment"),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        let header_item: &mut dyn HeaderItem = match (&self.service, &self.environment) {
            (None, None) => config,

            (Some(service_name), None) => {
                let Some(service) = config.get_service_mut(service_name) else {
                    return Err(HtrsError::new(format!("Unable to find service with name or alias `{}`", service_name).as_str()))
                };
                service
            },

            (Some(service_name), Some(environment_name)) => {
                let Some(service) = config.get_service_mut(service_name) else {
                    return Err(HtrsError::new(format!("Unable to find service with name or alias `{}`", service_name).as_str()))
                };
                let Some(environment) = service.get_environment_mut(environment_name) else {
                    return Err(HtrsError::new(format!("Unable to find environment with name or alias `{}` for service `{}`", environment_name, service.name).as_str()))
                };
                environment
            },

            _ => return Err(HtrsError::new("Invalid combination of arguments used"))
        };

        header_item.clear_header(self.header_name.clone());
        Ok(UpdateConfig)
    }
}
