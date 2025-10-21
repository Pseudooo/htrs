use crate::command_builder::MatchBinding;
use crate::config::{Environment, HtrsConfig};
use crate::outcomes::HtrsAction::PrintDialogue;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgMatches, Command};

pub struct ListEnvironmentsCommand {
    pub service: String,
    pub filter: Option<String>,
}

impl ListEnvironmentsCommand {
    pub fn get_command() -> Command {
        Command::new("environment")
            .about("List environments for a service")
            .visible_alias("env")
            .arg(
                Arg::new("service")
                    .help("Service to list environments for")
                    .required(true)
                    .long("service")
                    .short('s')
            )
            .arg(
                Arg::new("filter")
                    .help("Filter for environment name or alias")
                    .required(false)
                    .long("filter")
                    .short('f')
            )
    }

    pub fn bind_from_matches(args: &ArgMatches) -> ListEnvironmentsCommand {
        ListEnvironmentsCommand {
            service: args.bind_field("service"),
            filter: args.bind_field("filter"),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        let Some(service) = config.get_service(&self.service) else {
            return Err(HtrsError::new(format!("No service could be found with name or alias `{}`", self.service).as_str()));
        };

        let mut environments: Vec<Environment> = service.environments.to_vec();

        if let Some(filter) = &self.filter {
            environments.retain(|e| environment_matches_filter(e, filter));

            if environments.is_empty() {
                return Ok(PrintDialogue(format!("No environments found for service `{}` with name or alias containing `{}`", service.name, filter)))
            }
        }

        match environments.is_empty() {
            true => Ok(PrintDialogue("No environments defined".to_string())),
            false => Ok(PrintDialogue(
                environments.iter()
                    .map(|e| format!(" - {}", e.display_name()))
                    .collect::<Vec<String>>()
                    .join("\n")
            )),
        }
    }
}

fn environment_matches_filter(environment: &Environment, filter: &str) -> bool {
    if environment.name.to_lowercase().contains(filter) {
        return true;
    }

    if environment.alias.is_some() && environment.alias.as_ref().unwrap().to_lowercase().contains(filter) {
        return true;
    }

    false
}