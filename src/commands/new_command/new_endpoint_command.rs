use crate::command_builder::MatchBinding;
use crate::config::{Endpoint, HtrsConfig};
use crate::outcomes::HtrsAction::UpdateConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgAction, ArgMatches, Command};

pub struct NewEndpointCommand {
    pub service: String,
    pub name: String,
    pub path_template: String,
    pub query_parameters: Vec<String>,
}

impl NewEndpointCommand {
    pub fn get_command() -> Command {
        Command::new("endpoint")
            .arg(
                Arg::new("name")
                    .help("Name of the endpoint to create")
                    .required(true)
            )
            .arg(
                Arg::new("path")
                    .help("The path of the endpoint")
                    .required(true)
                    .long("path")
                    .short('p')
            )
            .arg(
                Arg::new("query")
                    .help("Query parameter for endpoint")
                    .required(false)
                    .action(ArgAction::Append)
                    .required(false)
                    .long("query")
                    .short('q')
            )
    }

    pub fn bind_from_matches(args: &ArgMatches) -> NewEndpointCommand {
        NewEndpointCommand {
            service: args.bind_field("service"),
            name: args.bind_field("name"),
            path_template: args.bind_field("path"),
            query_parameters: args.bind_field("query"),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        let Some(service) = config.get_service_mut(&self.service) else {
            return Err(HtrsError::new(format!("Unable to find service with name or alias `{}`", self.service).as_str()));
        };
        if service.get_endpoint(&self.name).is_some() {
            return Err(HtrsError::new(format!("Service `{}` already has an endpoint named `{}`", self.service, self.name).as_str()));
        }

        service.endpoints.push(Endpoint {
            name: self.name.clone(),
            path_template: self.path_template.clone(),
            query_parameters: self.query_parameters.clone(),
        });
        Ok(UpdateConfig)
    }
}
