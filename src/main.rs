mod command_args;
mod htrs_config;

use crate::command_args::RootCommands::{Call, Service};
use crate::command_args::ServiceCommands::{Add, Environment, Remove};
use crate::command_args::{CallOpts, Cli, EnvironmentCommands, ServiceCommands};
use crate::htrs_config::{HtrsConfig, ServiceConfig, ServiceEnvironmentConfig};
use clap::Parser;

fn main() {
    let parsed_args = Cli::parse();
    let mut config = HtrsConfig::load("./htrs_config.json");

    match parsed_args.command {
        Service(service_command) => {
            execute_service_command(&mut config, &service_command);
        },
        Call(options) => {
            execute_call_command(&config, options);
        },
    }
}

fn execute_service_command(config: &mut HtrsConfig, cmd: &ServiceCommands) {
    match cmd {
        Add { name } => {
            for service in config.services.iter() {
                if name.eq(service.name.as_str()) {
                    panic!("Service with name {} already exists", name)
                }
            }

            config.services.push(ServiceConfig::new(name.clone()));
            config.save("./htrs_config.json");
        },

        Remove { name } => {
            if config.service_defined(name) {
                config.services.retain(|x| !x.name.eq(name));
                println!("Service {} removed", name);
                config.save("./htrs_config.json");
            } else {
                panic!("Service with name {} not found", name)
            }
        }

        ServiceCommands::List => {
            if config.services.len() == 0 {
                println!("There are no services defined!");
            } else {
                for service in config.services.iter() {
                    println!("{}", service.name);
                }
            }
        },

        Environment(env_command) => {
            execute_environment_command(config, env_command);
        }
    }
}

fn execute_environment_command(config: &mut HtrsConfig, cmd: &EnvironmentCommands) {
    match cmd {
        EnvironmentCommands::Add { service_name, name: environment_name, host, default } => {
            if let Some(service) = config.find_service_config_mut(&service_name) {
                if service.environment_exists(&environment_name) {
                    panic!("Service {} already has an environment called {}", service_name, environment_name)
                } else {
                    service.environments.push(ServiceEnvironmentConfig::new(environment_name.clone(), host.clone(), default.clone()));
                    config.save("./htrs_config.json");
                }
            } else {
                panic!("Service {} not found", service_name)
            }
        },
        EnvironmentCommands::List { service_name } => {
            if let Some(service) = config.find_service_config(&service_name) {
                if service.environments.len() == 0 {
                    println!("There are no environments defined!");
                } else {
                    for environment in &service.environments {
                        if environment.default {
                            println!("{}: {} (default)", environment.name, environment.host);
                        } else {
                            println!("{}: {}", environment.name, environment.host);
                        }
                    }
                }
            } else {
                panic!("Service {} not found", service_name)
            }
        }
    }
}

fn execute_call_command(config: &HtrsConfig, cmd: CallOpts) {
    if let Some(service) = config.find_service_config(&cmd.service_name) {
        if let Some(environment_name) = cmd.environment {
            if let Some(environment) = service.find_environment(&environment_name) {
                let uri = format!("https://{}/", environment.host);
                make_get_request(&uri);
            } else {
                panic!("Service {} has no environment {}", cmd.service_name, environment_name);
            }
        } else if let Some(default_environment) = service.find_default_environment() {
            let uri = format!("https://{}/", default_environment.host);
            make_get_request(&uri);
        } else {
            panic!("No default environment found for service {}", cmd.service_name)
        }
    } else {
        panic!("Service not found");
    }
}

fn make_get_request(url: &str) {
    let client = reqwest::blocking::Client::new();
    match client.get(url).send() {
        Ok(response) => {
            println!("Receieved {} response", response.status());
        },
        Err(e) => {
            panic!("{}", e);
        }
    }
}
