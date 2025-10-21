use crate::commands::list_command::list_endpoint_command::ListEndpointsCommand;
use crate::commands::list_command::list_environment_command::ListEnvironmentsCommand;
use crate::commands::list_command::ListCommand::{Endpoint, Environment, Service};
use crate::config::HtrsConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{ArgMatches, Command};
use list_service_command::ListServicesCommand;

mod list_service_command;
mod list_environment_command;
mod list_endpoint_command;

pub enum ListCommand {
    Service(ListServicesCommand),
    Environment(ListEnvironmentsCommand),
    Endpoint(ListEndpointsCommand),
}

impl ListCommand {
    pub fn get_command() -> Command {
        Command::new("list")
            .about("List items defined in config")
            .visible_alias("ls")
            .arg_required_else_help(true)
            .subcommand(ListServicesCommand::get_command())
            .subcommand(ListEnvironmentsCommand::get_command())
            .subcommand(ListEndpointsCommand::get_command())
    }

    pub fn bind_from_matches(args: &ArgMatches) -> ListCommand {
        match args.subcommand() {
            Some(("service", service_matches)) => Service(ListServicesCommand::bind_from_matches(service_matches)),
            Some(("environment" | "env", environment_matches)) => Environment(ListEnvironmentsCommand::bind_from_matches(environment_matches)),
            Some(("endpoint", endpoint_matches)) => Endpoint(ListEndpointsCommand::bind_from_matches(endpoint_matches)),
            _ => unreachable!(),
        }
    }

    pub fn execute(&self, config: &mut HtrsConfig) -> Result<HtrsAction, HtrsError> {
        match self {
            Service(list_services_command) => list_services_command.execute(config),
            Environment(list_environments_command) => list_environments_command.execute(config),
            Endpoint(list_endpoints_command) => list_endpoints_command.execute(config),
        }
    }
}
