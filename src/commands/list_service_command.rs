use crate::command_builder::MatchBinding;
use crate::config::{HtrsConfig, Service};
use crate::outcomes::HtrsAction::PrintDialogue;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgMatches, Command};

pub struct ListServicesCommand {
    pub filter: Option<String>,
}

impl ListServicesCommand {
    pub fn get_command() -> Command {
        Command::new("service")
            .about("List all services")
            .arg(
                Arg::new("filter")
                    .help("Filter for service name or alias")
                    .required(false)
            )
    }

    pub fn bind_from_matches(args: &ArgMatches) -> ListServicesCommand {
        ListServicesCommand {
            filter: args.bind_field("filter"),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        let mut services: Vec<Service> = config.services
            .iter()
            .cloned()
            .collect();

        if let Some(filter) = &self.filter {
            services = services.into_iter().filter(|s| service_matches_filter(s, filter))
                .collect();

            if services.is_empty() {
                return Ok(PrintDialogue(format!("No services found with name or alias containing `{}`", filter)));
            }
        };

        match services.is_empty() {
            true => Ok(PrintDialogue("No services found".to_string())),
            false => Ok(PrintDialogue(
                services.iter()
                    .map(|s| format!(" - {}", s.display_name()))
                    .collect::<Vec<String>>()
                    .join("\n")
            )),
        }
    }
}

fn service_matches_filter(service: &Service, filter: &str) -> bool {
    if service.name.to_lowercase().contains(filter) {
        return true;
    }

    if service.alias.is_some() && service.alias.as_ref().unwrap().to_lowercase().contains(filter) {
        return true;
    }

    false
}
