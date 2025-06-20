mod command_args;
mod htrs_config;

use crate::command_args::RootCommands::{Call, Service};
use crate::command_args::ServiceCommands::{Add, Remove};
use crate::command_args::{CallOpts, Cli, ServiceCommands};
use crate::htrs_config::{HtrsConfig, ServiceConfig};
use clap::Parser;
use serde::{Deserialize, Serialize};

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
        Add { name, host } => {
            for service in config.services.iter() {
                if name.eq(service.name.as_str()) {
                    panic!("Service with name {} already exists", name)
                }
            }

            config.services.push(ServiceConfig::new(name.clone(), host.clone()));
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
                    println!("{}: {}", service.name, service.host);
                }
            }
        }
    }
}

fn execute_call_command(config: &HtrsConfig, cmd: CallOpts) {
    if let Some(service) = config.find_service_config(&cmd.name) {
        let client = reqwest::blocking::Client::new();
        match client.get(&service.host).send() {
            Ok(response) => {
                println!("Receieved {} response", response.status());
            },
            Err(e) => {
                panic!("{}", e);
            }
        }
    } else {
        panic!("Service not found");
    }
}
