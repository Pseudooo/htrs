use crate::command_builder::MatchBinding;
use crate::config::HtrsConfig;
use crate::outcomes::HtrsAction::UpdateConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgAction, ArgMatches, Command};

pub struct EditEndpointCommand {
    pub name: String,
    pub service: String,
    pub new_name: Option<String>,
    pub new_path: Option<String>,
    pub new_query_parameters: Vec<String>,
    pub delete_query_parameters: Vec<String>,
}

impl EditEndpointCommand {
    pub fn get_command() -> Command {
        Command::new("endpoint")
            .about("Edit an endpoint")
            .arg(
                Arg::new("name")
                    .help("Current name of the endpoint to edit")
                    .required(true)
            )
            .arg(
                Arg::new("service")
                    .help("The service containing the endpoint to target")
                    .required(true)
                    .long("service")
                    .short('s')
            )
            .arg(
                Arg::new("new_name")
                    .help("The new name for the endpoint")
                    .required(false)
                    .long("new-name")
            )
            .arg(
                Arg::new("new_path")
                    .help("The new path for the endpoint")
                    .required(false)
                    .long("new-path")
            )
            .arg(
                Arg::new("new_query")
                    .help("A new query parameter for the endpoint")
                    .required(false)
                    .action(ArgAction::Append)
                    .long("new-query")
            )
            .arg(
                Arg::new("delete_query")
                    .help("An existing query parameter to be removed")
                    .required(false)
                    .action(ArgAction::Append)
                    .long("del-query")
            )
    }

    pub fn bind_from_matches(args: &ArgMatches) -> EditEndpointCommand {
        EditEndpointCommand {
            name: args.bind_field("name"),
            service: args.bind_field("service"),
            new_name: args.bind_field("new_name"),
            new_path: args.bind_field("new_path"),
            new_query_parameters: args.bind_field("new_query"),
            delete_query_parameters: args.bind_field("delete_query"),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        let Some(service) = config.get_service_mut(&self.service) else {
            return Err(HtrsError::new(format!("No service could be found with name or alias `{}`", self.service).as_str()))
        };
        if service.get_endpoint(&self.name).is_none() {
            return Err(HtrsError::new(format!("No endpoint could be found with name `{}` for service `{}`", self.name, service.name).as_str()));
        }

        if let Some(new_name) = &self.new_name {
            if service.get_endpoint(new_name).is_some() {
                return Err(HtrsError::new(format!("An endpoint already exists with name `{}` for service `{}`", new_name, service.name).as_str()));
            }
        };

        let endpoint = service.get_endpoint_mut(&self.name).unwrap();

        if let Some(new_name) = &self.new_name {
            endpoint.name = new_name.clone();
        };
        if let Some(new_path) = &self.new_path {
            endpoint.path_template = new_path.clone();
        };
        if !self.new_query_parameters.is_empty() {
            endpoint.query_parameters.extend(self.new_query_parameters.iter().cloned());
        }
        if !self.delete_query_parameters.is_empty() {
            endpoint.query_parameters = endpoint.query_parameters.iter()
                .filter(|q| !self.delete_query_parameters.contains(q))
                .cloned()
                .collect();
        }
        Ok(UpdateConfig)
    }
}
