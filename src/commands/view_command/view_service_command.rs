use crate::commands::bindings::MatchBinding;
use crate::config::current_config::{Endpoint, Environment, HtrsConfig};
use crate::outcomes::HtrsAction::PrintDialogue;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::{Arg, ArgMatches, Command};

pub struct ViewServiceCommand {
    pub name: String,
}

impl ViewServiceCommand {
    pub fn get_command() -> Command {
        Command::new("service")
            .arg_required_else_help(true)
            .arg(
                Arg::new("name")
                    .required(true)
                    .help("Name or alias of the service to view")
            )
    }

    pub fn bind_from_matches(args: &ArgMatches) -> Self {
        Self {
            name: args.bind_field("name"),
        }
    }

    pub fn execute(&self, config: &HtrsConfig) -> Result<HtrsAction, HtrsError> {
        let Some(service) = config.get_service(&self.name) else {
            return Err(HtrsError::new(format!("No service could be found with name or alias `{}`", self.name).as_str()));
        };

        let mut text = String::new();
        text.push_str(format!("Name: {}\n", service.name).as_str());
        if let Some(alias) = &service.alias {
            text.push_str(format!(" Alias: {}\n", alias).as_str());
        }
        text.push_str("Environments:\n");
        let environment_text = service.environments.iter()
            .map(Self::get_environment_str)
            .collect::<Vec<String>>()
            .join("");
        text.push_str(match environment_text.is_empty() {
            true => "  (no environments)\n",
            false => environment_text.as_str()
        });

        text.push_str("Endpoints:\n");
        let endpoint_text = service.endpoints.iter()
            .map(Self::get_endpoint_string)
            .collect::<Vec<String>>()
            .join("");
        text.push_str(match endpoint_text.is_empty() {
            true => "  (no endpoints)\n",
            false => endpoint_text.as_str(),
        });

        Ok(PrintDialogue(text))
    }

    fn get_environment_str(environment: &Environment) -> String {
        match environment.alias {
            Some(ref alias) => format!(" - {} ({}) ~ {}\n", environment.name, alias, environment.host),
            None => format!(" - {} ~ {}\n", environment.name, environment.host)
        }
    }

    fn get_endpoint_string(endpoint: &Endpoint) -> String {
        let mut text = String::new();
        text.push_str(format!(" - {} ~ {}\n", endpoint.name, endpoint.path_template).as_str());
        for param in &endpoint.query_parameters {
            match param.required {
                true => text.push_str(format!("   - *{}\n", param.name).as_str()),
                false => text.push_str(format!("   - {}\n", param.name).as_str()),
            };
        }

        text
    }
}
