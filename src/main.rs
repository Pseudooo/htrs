mod command_args;
mod htrs_config;

use crate::command_args::RootCommands::{Call, Service};
use crate::command_args::ServiceCommands::{Add, Environment, Remove};
use crate::command_args::{CallOpts, Cli, EnvironmentCommands, ServiceCommands};
use crate::htrs_config::{HtrsConfig, ServiceConfig, ServiceEnvironmentConfig};
use clap::Parser;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct HtrsError {
    details: String
}

impl HtrsError {
    fn new(msg: &str) -> HtrsError {
        HtrsError { details: msg.to_string() }
    }
}

impl fmt::Display for HtrsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for HtrsError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn main() {
    let parsed_args = Cli::parse();
    let mut config = HtrsConfig::load("./htrs_config.json");

    let result = match parsed_args.command {
        Service(service_command) => {
            execute_service_command(&mut config, &service_command)
        },
        Call(options) => {
            execute_call_command(&config, options)
        },
    };

    if let Err(e) = result {
        println!("{}", e.details);
    }
}

fn execute_service_command(config: &mut HtrsConfig, cmd: &ServiceCommands) -> Result<(), HtrsError> {
    match cmd {
        Add { name } => {
            for service in config.services.iter() {
                if name.eq(service.name.as_str()) {
                    return Err(HtrsError::new(&format!("Service \"{name}\" already exists")))
                }
            }

            config.services.push(ServiceConfig::new(name.clone()));
            config.save("./htrs_config.json");
            println!("Service \"{name}\" created");
            Ok(())
        },

        Remove { name } => {
            if config.service_defined(name) {
                config.services.retain(|x| !x.name.eq(name));
                println!("Service {} removed", name);
                config.save("./htrs_config.json");
                Ok(())
            } else {
                Err(HtrsError::new(&format!("Service \"{name}\" does not exist")))
            }
        }

        ServiceCommands::List => {
            if config.services.len() == 0 {
                println!("No services defined");
            } else {
                for service in &config.services {
                    println!(" - {}", service.name);
                }
            }
            Ok(())
        },

        Environment(env_command) => {
            execute_environment_command(config, env_command)
        }
    }
}

fn execute_environment_command(config: &mut HtrsConfig, cmd: &EnvironmentCommands) -> Result<(), HtrsError> {
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
                    config.save("./htrs_config.json");
                    println!("Environment {environment_name} created for {service_name}");
                    Ok(())
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
                    for environment in &service.environments {
                        if environment.default {
                            println!("{}: {} (default)", environment.name, environment.host);
                        } else {
                            println!("{}: {}", environment.name, environment.host);
                        }
                    }
                    Ok(())
                }
            } else {
                Err(HtrsError::new(&format!("Service {service_name} does not exist")))
            }
        },

        EnvironmentCommands::Remove { service_name, environment_name } => {
            if let Some(service) = config.find_service_config_mut(&service_name) {
                service.remove_environment(environment_name);
                println!("Environment {environment_name} removed for {service_name}");
                Ok(())
            } else {
                Err(HtrsError::new(&format!("Service {service_name} does not exist")))
            }
        }
    }
}

fn execute_call_command(config: &HtrsConfig, cmd: CallOpts) -> Result<(), HtrsError> {
    if let Some(service) = config.find_service_config(&cmd.service_name) {
        if let Some(environment_name) = cmd.environment {
            if let Some(environment) = service.find_environment(&environment_name) {
                let uri = format!("https://{}/", environment.host);
                make_get_request(&uri)
            } else {
                Err(HtrsError::new(&format!("No environments defined for {}", service.name)))
            }
        } else if let Some(default_environment) = service.find_default_environment() {
            let uri = format!("https://{}/", default_environment.host);
            make_get_request(&uri)
        } else {
            Err(HtrsError::new(&format!("No default environment defined for {}", cmd.service_name)))
        }
    } else {
        Err(HtrsError::new(&format!("Service {} does not exist", cmd.service_name)))
    }
}

fn make_get_request(url: &str) -> Result<(), HtrsError> {
    let client = reqwest::blocking::Client::new();
    match client.get(url).send() {
        Ok(response) => {
            println!("Receieved {} response", response.status());
            Ok(())
        },
        Err(e) => {
            Err(HtrsError::new(&format!("Failed to call {} response: {}", url, e)))
        }
    }
}
