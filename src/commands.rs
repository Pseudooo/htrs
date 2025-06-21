use crate::command_args::RootCommands::{Call, Service};
use crate::command_args::ServiceCommands::{Add, Environment, Remove};
use crate::command_args::{CallOpts, Cli, EnvironmentCommands, ServiceCommands};
use crate::htrs_config::{HtrsConfig, ServiceConfig, ServiceEnvironmentConfig};
use crate::{HtrsError, HtrsOutcome};
use reqwest::blocking::Response;

pub fn execute_command(config: &mut HtrsConfig, cmd: Cli) -> Result<HtrsOutcome, HtrsError> {
    match cmd.command {
        Service(service_command) => {
            execute_service_command(config, &service_command)
        },
        Call(options) => {
            execute_call_command(config, options)
        },
    }
}

fn execute_service_command<'a>(config: &'a mut HtrsConfig, cmd: &ServiceCommands) -> Result<HtrsOutcome<'a>, HtrsError> {
    match cmd {
        Add { name } => {
            for service in config.services.iter() {
                if name.eq(service.name.as_str()) {
                    return Err(HtrsError::new(&format!("Service \"{name}\" already exists")))
                }
            }

            config.services.push(ServiceConfig::new(name.clone()));
            Ok(HtrsOutcome::new(
                config,
                true,
                format!("Service \"{name}\" created")
            ))
        },

        Remove { name } => {
            if config.service_defined(name) {
                config.services.retain(|x| !x.name.eq(name));
                println!("Service {} removed", name);
                config.save("./htrs_config.json");
                Ok(HtrsOutcome::new(
                    config,
                    true,
                    format!("Service \"{name}\" removed"),
                ))
            } else {
                Err(HtrsError::new(&format!("Service \"{name}\" does not exist")))
            }
        }

        ServiceCommands::List => match config.services.len() {
            0 => Ok(HtrsOutcome::new(
                config,
                false,
                "No services found".to_string(),
            )),
            _ => Ok(HtrsOutcome::new(
                config,
                false,
                format!(" - {}", config.services.iter().map(|service| service.name.clone())
                    .collect::<Vec<String>>()
                    .join("\n - ")),
            ))
        },

        Environment(env_command) => {
            execute_environment_command(config, env_command)
        }
    }
}

fn execute_environment_command<'a>(config: &'a mut HtrsConfig, cmd: &EnvironmentCommands) -> Result<HtrsOutcome<'a>, HtrsError> {
    match cmd {
        EnvironmentCommands::Add { service_name, name: environment_name, host, default } => {
            if let Some(service) = config.find_service_config_mut(&service_name) {
                if service.environment_exists(&environment_name) {
                    Err(HtrsError::new(&format!("{environment_name} already defined under {service_name}")))
                } else {
                    if *default {
                        if let Some(default_environment) = service.find_default_environment_mut() {
                            default_environment.default = false;
                        }
                    }

                    service.environments.push(ServiceEnvironmentConfig::new(environment_name.clone(), host.clone(), default.clone()));
                    Ok(HtrsOutcome::new(
                        config,
                        true,
                        format!("Environment \"{environment_name}\" created for {service_name}"),
                    ))
                }
            } else {
                Err(HtrsError::new(&format!("Service {service_name} does not exist")))
            }
        },

        EnvironmentCommands::List { service_name } => {
            if let Some(service) = config.find_service_config(&service_name) {
                if service.environments.len() == 0 {
                    Err(HtrsError::new(&format!("No environments defined for {service_name}")))
                } else {
                    let environment_list = service.environments.iter()
                        .map(|env| match env.default {
                            true => format!(" - {}: {} (default)", env.name, env.host),
                            false => format!(" - {}: {}", env.name, env.host),
                        })
                        .collect::<Vec<String>>()
                        .join("\n");

                    Ok(HtrsOutcome::new(
                        config,
                        false,
                        environment_list,
                    ))
                }
            } else {
                Err(HtrsError::new(&format!("Service {service_name} does not exist")))
            }
        },

        EnvironmentCommands::Remove { service_name, environment_name } => {
            if let Some(service) = config.find_service_config_mut(&service_name) {
                service.remove_environment(environment_name);
                Ok(HtrsOutcome::new(
                    config,
                    true,
                    format!("Environment {environment_name} removed for {service_name}"),
                ))
            } else {
                Err(HtrsError::new(&format!("Service {service_name} does not exist")))
            }
        }
    }
}

fn execute_call_command(config: &HtrsConfig, cmd: CallOpts) -> Result<HtrsOutcome, HtrsError> {
    if let Some(service) = config.find_service_config(&cmd.service_name) {
        if let Some(environment_name) = cmd.environment {
            if let Some(environment) = service.find_environment(&environment_name) {
                let uri = format!("https://{}/", environment.host);
                match make_get_request(&uri) {
                    Ok(response) => Ok(HtrsOutcome::new(
                        config,
                        false,
                        format!("Received {} response", response.status()),
                    )),
                    Err(e) => Err(e),
                }
            } else {
                Err(HtrsError::new(&format!("No environments defined for {}", service.name)))
            }
        } else if let Some(default_environment) = service.find_default_environment() {
            let uri = format!("https://{}/", default_environment.host);
            match make_get_request(&uri) {
                Ok(response) => Ok(HtrsOutcome::new(
                    config,
                    false,
                    format!("Received {} response", response.status()),
                )),
                Err(e) => Err(e),
            }
        } else {
            Err(HtrsError::new(&format!("No default environment defined for {}", cmd.service_name)))
        }
    } else {
        Err(HtrsError::new(&format!("Service {} does not exist", cmd.service_name)))
    }
}

fn make_get_request(url: &str) -> Result<Response, HtrsError> {
    let client = reqwest::blocking::Client::new();
    match client.get(url).send() {
        Ok(response) => Ok(response),
        Err(e) => {
            Err(HtrsError::new(&format!("Failed to call {} response: {}", url, e)))
        }
    }
}

#[cfg(test)]
mod service_command_tests {
    use super::*;

    #[test]
    fn given_new_service_when_create_config_updated() {
        // Arrange
        let mut config = HtrsConfig::new();
        let command = Cli {
            command: Service(
                Add {
                    name: "foo".to_string(),
                }
            )
        };

        // Act
        let result = execute_command(&mut config, command);

        // Assert
        assert!(result.is_ok());
        let outcome = result.unwrap();
        assert_eq!(outcome.config_updated, true);
        let updated_config = outcome.config;
        assert_eq!(updated_config.services.len(), 1);
        assert_eq!(updated_config.services[0].name, "foo".to_string());
        assert_eq!(updated_config.services[0].environments.len(), 0);
    }

    #[test]
    fn given_existing_service_when_create_no_action_with_error() {
        // Arrange
        let mut config = HtrsConfig::new();
        config.services.push(ServiceConfig::new("foo".to_string()));
        let command = Cli {
            command: Service(
                Add {
                    name: "foo".to_string(),
                }
            )
        };

        // Act
        let result = execute_command(&mut config, command);

        // Assert
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_ne!(error.details.len(), 0);
    }
}