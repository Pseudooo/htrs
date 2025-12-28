use crate::commands::bindings::MatchBinding;
use crate::config::current_config::{Endpoint, HtrsConfig};
use crate::outcomes::HtrsAction::PrintDialogue;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgMatches, Command};

pub struct ListEndpointsCommand {
    pub service: String,
    pub filter: Option<String>,
}

impl ListEndpointsCommand {
    pub fn get_command() -> Command {
        Command::new("endpoint")
            .about("List endpoints for a service")
            .arg(
                Arg::new("service")
                    .help("Service to list environments for")
                    .required(true)
                    .long("service")
                    .short('s')
            )
            .arg(
                Arg::new("filter")
                    .help("Filter for endpoint name")
                    .required(false)
                    .long("filter")
                    .short('f')
            )
    }

    pub fn bind_from_matches(args: &ArgMatches) -> ListEndpointsCommand {
        ListEndpointsCommand {
            service: args.bind_field("service"),
            filter: args.bind_field("filter"),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        let Some(service) = config.get_service(&self.service) else {
            return Err(HtrsError::new(format!("No service could be found with name or alias `{}`", self.service).as_str()))
        };

        let mut endpoints: Vec<Endpoint> = service.endpoints.to_vec();

        if let Some(filter) = &self.filter {
            endpoints.retain(|e| e.name.to_lowercase().contains(filter));

            if endpoints.is_empty() {
                return Ok(PrintDialogue(format!("No endpoints found for service `{}` with name containing `{}`", service.name, filter)));
            }
        }

        match endpoints.is_empty() {
            true => Ok(PrintDialogue("No endpoints defined".to_string())),
            false => Ok(PrintDialogue(
                endpoints.iter()
                    .map(|e| format!(" - {}", e.name))
                    .collect::<Vec<String>>()
                    .join("\n")
            ))
        }
    }
}
