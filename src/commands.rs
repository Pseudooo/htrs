use crate::command_args::RootCommands::{Call, Service};
use crate::command_args::ServiceCommands::{Add, Environment, Remove};
use crate::command_args::{CallServiceOptions, EnvironmentCommands, RootCommands, ServiceCommands};
use crate::htrs_config::{HtrsConfig, ServiceConfig, ServiceEnvironmentConfig};
use crate::outcomes::{HtrsError, HtrsOutcome};
use reqwest::blocking::{Client, Request, Response};
use reqwest::{Method, Url};
use std::collections::HashMap;

pub fn execute_command(config: &mut HtrsConfig, cmd: RootCommands) -> Result<HtrsOutcome, HtrsError> {
    match cmd {
        Service(service_command) => {
            execute_service_command(config, &service_command)
        },
        Call(options) => {
            execute_call_command(config, options)
        },
        _ => panic!("BAD")
    }
}

fn execute_service_command(config: &mut HtrsConfig, cmd: &ServiceCommands) -> Result<HtrsOutcome, HtrsError> {
    match cmd {
        Add { name } => {
            for service in config.services.iter() {
                if name.eq(service.name.as_str()) {
                    return Err(HtrsError::new(&format!("Service \"{name}\" already exists")))
                }
            }

            config.services.push(ServiceConfig::new(name.clone()));
            Ok(HtrsOutcome::new(
                true,
                format!("Service \"{name}\" created"),
                None
            ))
        },

        Remove { name } => {
            if config.service_defined(name) {
                config.services.retain(|x| !x.name.eq(name));
                Ok(HtrsOutcome::new(
                    true,
                    format!("Service \"{name}\" removed"),
                    None
                ))
            } else {
                Err(HtrsError::new(&format!("Service \"{name}\" does not exist")))
            }
        }

        ServiceCommands::List => match config.services.len() {
            0 => Ok(HtrsOutcome::new(
                false,
                "No services found".to_string(),
                None)),
            _ => Ok(HtrsOutcome::new(
                false,
                format!(" - {}", config.services.iter().map(|service| service.name.clone())
                    .collect::<Vec<String>>()
                    .join("\n - ")),
                None)),
        },

        Environment(env_command) => {
            execute_environment_command(config, env_command)
        }
    }
}

fn execute_environment_command(config: &mut HtrsConfig, cmd: &EnvironmentCommands) -> Result<HtrsOutcome, HtrsError> {
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
                        true,
                        format!("Environment \"{environment_name}\" created for {service_name}"),
                        None
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
                        false,
                        environment_list,
                        None
                    ))
                }
            } else {
                Err(HtrsError::new(&format!("Service {service_name} does not exist")))
            }
        },

        EnvironmentCommands::Remove { service_name, environment_name } => {
            if let Some(service) = config.find_service_config_mut(&service_name) {
                match service.remove_environment(environment_name) {
                    true => Ok(HtrsOutcome::new(
                        true,
                        format!("Environment {environment_name} removed for {service_name}"),
                        None
                    )),
                    false => Err(HtrsError::new(&format!("Environment {environment_name} does not exist")))
                }
            } else {
                Err(HtrsError::new(&format!("Service {service_name} does not exist")))
            }
        }
    }
}

fn execute_call_command(config: &HtrsConfig, cmd: CallServiceOptions) -> Result<HtrsOutcome, HtrsError> {
    if let Some(service) = config.find_service_config(&cmd.service) {
        let environment: &ServiceEnvironmentConfig;
        if let Some(environment_name) = cmd.environment {
            if let Some(named_environment) = service.find_environment(&environment_name) {
                environment = named_environment;
            } else {
                return Err(HtrsError::new(&format!("No environments defined for {}", service.name)));
            }
        } else if let Some(default_environment) = service.find_default_environment() {
            environment = default_environment;
        } else {
            return Err(HtrsError::new(&format!("No default environment defined for {}", cmd.service)));
        }

        let mut headers: HashMap<String, String> = HashMap::new();
        for header_kvp in cmd.header {
            let parts = header_kvp.splitn(2, '=').collect::<Vec<&str>>();
            if let [key, value] = parts.as_slice() {
                headers.insert(key.to_string(), value.to_string());
            } else {
                return Err(HtrsError::new(&format!("Header {} is invalid", header_kvp)))
            }
        }

        let request = match build_request(&environment.host, cmd.path, cmd.query, headers) {
            Ok(req) => req,
            Err(e) => return Err(e),
        };
        let response = make_get_request(request)?;
        Ok(HtrsOutcome::new(
            false,
            format!("Received {} response",response.status()),
            None))
    } else {
        Err(HtrsError::new(&format!("Service {} does not exist", cmd.service)))
    }
}

fn build_request(host: &str, path: Option<String>, query: Vec<String>, headers: HashMap<String, String>) -> Result<Request, HtrsError> {
    let mut url = match Url::parse(&format!("https://{host}")) {
        Ok(uri) => uri,
        Err(e) => return Err(HtrsError::new(&e.to_string())),
    };

    url = match path {
        Some(path) => match url.join(&path) {
            Ok(uri) => uri,
            Err(e) => return Err(HtrsError::new(&e.to_string())),
        },
        None => url,
    };

    url = match url.join(&format!("?{}", query.join("&"))) {
        Ok(uri) => uri,
        Err(e) => return Err(HtrsError::new(&e.to_string())),
    };

    let mut builder = Client::new().request(Method::GET, url);
    for (key, value) in headers {
        builder = builder.header(key, value);
    }

    let request = match builder.build() {
        Ok(req) => req,
        Err(e) => return Err(HtrsError::new(&e.to_string())),
    };

    Ok(request)
}

fn make_get_request(request: Request) -> Result<Response, HtrsError> {
    let result = Client::new().execute(request);
    match result {
        Ok(response) => Ok(response),
        Err(e) => {
            Err(HtrsError::new(&format!("Failed to make request: {}", e)))
        }
    }
}

#[cfg(test)]
mod service_command_tests {
    use super::*;
    use crate::command_args::ServiceCommands::List;
    use rstest::rstest;

    #[test]
    fn given_new_service_when_create_then_config_updated_with_result() {
        // Arrange
        let mut config = HtrsConfig::new();
        config.services.push(ServiceConfig::new("foo".to_string()));
        let command = Service(
            Add {
                name: "bar".to_string(),
            }
        );

        // Act
        let result = execute_command(&mut config, command);

        // Assert
        assert!(result.is_ok());
        let outcome = result.unwrap();
        assert_eq!(outcome.config_updated, true);
        assert_eq!(config.services.len(), 2);
        assert!(config.services.iter().any(|s| s.name == "foo" && s.environments.len() == 0));
        assert!(config.services.iter().any(|s| s.name == "bar" && s.environments.len() == 0));
    }

    #[test]
    fn given_existing_service_when_create_then_no_update_with_error() {
        // Arrange
        let mut config = HtrsConfig::new();
        config.services.push(ServiceConfig::new("foo".to_string()));
        let command = Service(
            Add {
                name: "foo".to_string(),
            }
        );

        // Act
        let result = execute_command(&mut config, command);

        // Assert
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_ne!(error.details.len(), 0);
    }

    #[test]
    fn given_existing_service_when_remove_then_config_updated_with_result() {
        // Arrange
        let mut config = HtrsConfig::new();
        config.services.push(ServiceConfig::new("foo".to_string()));
        config.services.push(ServiceConfig::new("bar".to_string()));
        let command = Service(
            Remove {
                name: "foo".to_string(),
            },
        );

        // Act
        let result = execute_command(&mut config, command);

        assert!(result.is_ok());
        let outcome = result.unwrap();
        assert!(outcome.config_updated);
        assert!(!config.services.iter().any(|s| s.name == "foo"));
    }

    #[test]
    fn given_unknown_service_when_remove_then_no_update_with_error() {
        // Arrange
        let mut config = HtrsConfig::new();
        config.services.push(ServiceConfig::new("foo".to_string()));
        let command = Service(
            Remove {
                name: "bar".to_string(),
            },
        );

        // Act
        let result = execute_command(&mut config, command);

        // Assert
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_ne!(error.details.len(), 0);
    }

    #[test]
    fn given_no_services_when_list_then_no_update_with_result() {
        // Arrange
        let mut config = HtrsConfig::new();
        let command = Service(List);

        // Act
        let result = execute_command(&mut config, command);

        // Assert
        assert!(result.is_ok());
        let outcome = result.unwrap();
        assert_eq!(outcome.config_updated, false);
        assert_ne!(outcome.outcome_dialogue.len(), 0);
    }

    #[test]
    fn given_known_services_when_list_then_no_update_with_result() {
        // Arrange
        let mut config = HtrsConfig::new();
        config.services.push(ServiceConfig::new("foo".to_string()));
        config.services.push(ServiceConfig::new("bar".to_string()));
        let command = Service(List);

        // Act
        let result = execute_command(&mut config, command);

        // Assert
        assert!(result.is_ok());
        let outcome = result.unwrap();
        assert_eq!(outcome.config_updated, false);
        assert_ne!(outcome.outcome_dialogue.len(), 0);
    }

    #[test]
    fn given_unknown_service_when_add_environment_then_no_update_with_error() {
        // Arrange
        let mut config = HtrsConfig::new();
        config.services.push(ServiceConfig::new("foo".to_string()));
        let command = Service(
            Environment(
                EnvironmentCommands::Add {
                    service_name: "bar".to_string(),
                    name: "kek".to_string(),
                    host: "google.com".to_string(),
                    default: false,
                },
            ),
        );

        // Act
        let result = execute_command(&mut config, command);

        // Assert
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_ne!(error.details.len(), 0);
    }

    #[rstest]
    #[case(true)]
    #[case(false)]
    fn given_known_service_when_add_environment_then_update_with_result(#[case] is_default: bool) {
        // Arrange
        let mut config = HtrsConfig::new();
        config.services.push(ServiceConfig::new("foo".to_string()));
        let command = Service(
            Environment(
                EnvironmentCommands::Add {
                    service_name: "foo".to_string(),
                    name: "bar".to_string(),
                    host: "google.com".to_string(),
                    default: is_default,
                },
            ),
        );

        // Act
        let result = execute_command(&mut config, command);

        // Assert
        assert!(result.is_ok());
        let outcome = result.unwrap();
        assert!(outcome.config_updated);
        let updated_service_option = config.services.iter().find(|s| s.name == "foo");
        assert!(updated_service_option.is_some());
        let updated_service = updated_service_option.unwrap();
        assert_eq!(updated_service.environments.len(), 1);
        assert!(updated_service.environments.iter().any(
            |s| s.name == "bar" && s.host == "google.com" && s.default == is_default));
    }

    #[test]
    fn given_known_service_with_default_environment_when_add_new_default_then_existing_replaced_with_result() {
        // Arrange
        let mut service = ServiceConfig::new("foo".to_string());
        service.environments.push(ServiceEnvironmentConfig::new(
            "bar".to_string(),
            "google.com".to_string(),
            true));
        let mut config = HtrsConfig::new();
        config.services.push(service);
        let command = Service(
            Environment(
                EnvironmentCommands::Add {
                    service_name: "foo".to_string(),
                    name: "kek".to_string(),
                    host: "gmail.com".to_string(),
                    default: true,
                },
            ),
        );

        // Act
        let result = execute_command(&mut config, command);

        // Assert
        assert!(result.is_ok());
        let outcome = result.unwrap();
        assert!(outcome.config_updated);
        let service = config.services.iter().find(|s| s.name == "foo");
        assert!(service.is_some());
        let service = service.unwrap();
        assert!(service.environments.iter().any(|s| s.name == "bar" && !s.default));
        assert!(service.environments.iter().any(|s| s.name == "kek" && s.default));
    }

    #[test]
    fn given_known_service_with_existing_environment_when_create_then_no_update_with_error() {
        // Arrange
        let mut service = ServiceConfig::new("foo".to_string());
        service.environments.push(ServiceEnvironmentConfig::new(
            "bar".to_string(),
            "google.com".to_string(),
            true));
        let mut config = HtrsConfig::new();
        config.services.push(service);
        let command = Service(
            Environment(
                EnvironmentCommands::Add {
                    service_name: "foo".to_string(),
                    name: "bar".to_string(),
                    host: "google.com".to_string(),
                    default: false,
                },
            ),
        );

        // Act
        let result = execute_command(&mut config, command);

        // Assert
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_ne!(error.details.len(), 0);
    }

    #[test]
    fn given_unknown_service_when_list_environments_then_no_update_with_error() {
        // Arrange
        let mut service = ServiceConfig::new("foo".to_string());
        service.environments.push(ServiceEnvironmentConfig::new(
            "bar".to_string(),
            "google".to_string(),
            false));
        let mut config = HtrsConfig::new();
        config.services.push(service);
        let command = Service(
            Environment(
                EnvironmentCommands::List {
                    service_name: "kek".to_string(),
                },
            ),
        );

        // Act
        let result = execute_command(&mut config, command);

        // Assert
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_ne!(error.details.len(), 0);
    }

    #[test]
    fn given_known_service_when_list_environments_then_no_update_with_result() {
        // Arrange
        let mut service = ServiceConfig::new("foo".to_string());
        service.environments.push(ServiceEnvironmentConfig::new(
            "bar".to_string(),
            "google".to_string(),
            false));
        let mut config = HtrsConfig::new();
        config.services.push(service);
        let command = Service(
            Environment(
                EnvironmentCommands::List {
                    service_name: "foo".to_string(),
                },
            ),
        );

        // Act
        let result = execute_command(&mut config, command);

        // Assert
        assert!(result.is_ok());
        let outcome = result.unwrap();
        assert_eq!(outcome.config_updated, false);
        assert_ne!(outcome.outcome_dialogue.len(), 0);
    }

    #[test]
    fn given_unknown_service_when_remove_environment_then_error() {
        // Arrange
        let mut service = ServiceConfig::new("foo".to_string());
        service.environments.push(ServiceEnvironmentConfig::new(
            "bar".to_string(),
            "google".to_string(),
            false));
        let mut config = HtrsConfig::new();
        config.services.push(service);
        let command = Service(
            Environment(
                EnvironmentCommands::Remove {
                    service_name: "kek".to_string(),
                    environment_name: "lmao".to_string(),
                },
            ),
        );

        // Act
        let result = execute_command(&mut config, command);

        // Assert
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_ne!(error.details.len(), 0);
    }

    #[test]
    fn given_known_service_when_remove_unknown_environment_then_error() {
        // Arrange
        let mut service = ServiceConfig::new("foo".to_string());
        service.environments.push(ServiceEnvironmentConfig::new(
            "bar".to_string(),
            "google".to_string(),
            false));
        let mut config = HtrsConfig::new();
        config.services.push(service);
        let command = Service(
            Environment(
                EnvironmentCommands::Remove {
                    service_name: "foo".to_string(),
                    environment_name: "lmao".to_string(),
                },
            ),
        );

        // Act
        let result = execute_command(&mut config, command);

        // Assert
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_ne!(error.details.len(), 0);
    }

    #[test]
    fn given_known_service_when_remove_known_environment_then_update_with_result() {
        // Arrange
        let mut service = ServiceConfig::new("foo".to_string());
        service.environments.push(ServiceEnvironmentConfig::new(
            "bar".to_string(),
            "google".to_string(),
            false));
        let mut config = HtrsConfig::new();
        config.services.push(service);
        let command = Service(
            Environment(
                EnvironmentCommands::Remove {
                    service_name: "foo".to_string(),
                    environment_name: "bar".to_string(),
                },
            ),
        );

        // Act
        let result = execute_command(&mut config, command);

        // Assert
        assert!(result.is_ok());
        let outcome = result.unwrap();
        assert!(outcome.config_updated);
        let updated_service = config.services.iter()
            .find(|s| s.name == "foo");
        assert!(updated_service.is_some());
        let updated_service = updated_service.unwrap();
        assert_eq!(updated_service.environments.len(), 0);
    }
}