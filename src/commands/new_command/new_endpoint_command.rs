use crate::commands::bindings::MatchBinding;
use crate::common::{get_duplicates_from_vec, get_params_from_path};
use crate::config::{Endpoint, HtrsConfig, QueryParameter};
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
            .arg(
                Arg::new("service")
                    .help("The service endpoint will be created for")
                    .required(true)
                    .long("service")
                    .short('s')
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
        if self.name.is_empty() {
            return Err(HtrsError::new("Endpoint name cannot be empty"));
        }
        if self.path_template.is_empty() {
            return Err(HtrsError::new("Endpoint path cannot be empty"));
        }
        let path_parameters = get_params_from_path(&self.path_template);
        let duplicates = get_duplicates_from_vec(path_parameters);
        if !duplicates.is_empty() {
            return Err(HtrsError::new(format!("The following path parameters were used more than once: {}", duplicates.join(",")).as_str()));
        }

        let query_params: Vec<QueryParameter> = self.query_parameters.iter()
            .map(|q| QueryParameter::from_shorthand(q))
            .collect();
        if query_params.iter().any(|q| q.name.is_empty()) {
            return Err(HtrsError::new("Query parameter names must not be blank"));
        }

        let query_param_names: Vec<String> = query_params.iter()
            .map(|q| q.name.clone())
            .collect();
        let duplicate_query_params = get_duplicates_from_vec(query_param_names);
        if !duplicate_query_params.is_empty() {
            return Err(HtrsError::new(format!("The following query parameter names were used more than once: {}", duplicate_query_params.join(",")).as_str()));
        }

        let Some(service) = config.get_service_mut(&self.service) else {
            return Err(HtrsError::new(format!("Unable to find service with name or alias `{}`", self.service).as_str()));
        };
        if service.get_endpoint(&self.name).is_some() {
            return Err(HtrsError::new(format!("Service `{}` already has an endpoint named `{}`", self.service, self.name).as_str()));
        }

        service.endpoints.push(Endpoint {
            name: self.name.clone(),
            path_template: self.path_template.clone(),
            query_parameters: self.query_parameters.iter().map(|q| QueryParameter::from_shorthand(q)).collect(),
        });
        Ok(UpdateConfig)
    }
}
