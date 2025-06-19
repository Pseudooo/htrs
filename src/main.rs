use crate::RootCommands::Service;
use crate::ServiceCommands::Add;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::path::Path;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: RootCommands
}

#[derive(Subcommand)]
enum RootCommands {
    #[command(subcommand)]
    Service(ServiceCommands),
    Call,
}

#[derive(Subcommand)]
enum ServiceCommands {
    Add {
        #[arg(long, value_name = "name")]
        name: String,

        #[arg(long, value_name = "url")]
        host: String,
    },
    List,
}

#[derive(Serialize, Deserialize)]
struct HtrsConfig {
    services: Vec<ServiceConfig>,
}

impl HtrsConfig {
    fn new() -> HtrsConfig {
        HtrsConfig { services: Vec::new() }
    }
}

#[derive(Serialize, Deserialize)]
struct ServiceConfig {
    name: String,
    host: String,
}

impl ServiceConfig {
    fn new(name: String, host: String) -> ServiceConfig {
        ServiceConfig { name, host }
    }
}

fn main() {
    let parsed_args = Cli::parse();
    match parsed_args.command {
        Service(service_command) => {
            execute_service_command(&service_command);
        },
        _ => panic!("NO"),
    }

    // let client = reqwest::blocking::Client::new();
    // let response_result = client.get(&parsed_args.url).send();
    // match response_result {
    //     Ok(response) => {
    //         println!("Response: {}", response.status());
    //     },
    //     Err(error) => {
    //         panic!("{}", error);
    //     }
    // }
}

fn execute_service_command(cmd: &ServiceCommands) {
    let mut config = ensure_config();

    match cmd {
        Add { name, host } => {
            for service in config.services.iter() {
                if name.eq(service.name.as_str()) {
                    panic!("Service with name {} already exists", name)
                }
            }

            config.services.push(ServiceConfig::new(name.clone(), host.clone()));

            save_config(config);
        },
        ServiceCommands::List => {
            for service in config.services.iter() {
                println!("{}: {}", service.name, service.host);
            }
        }
    }
}

fn ensure_config() -> HtrsConfig {
    let config_path = Path::new("./htrs_config.json");
    if config_path.exists() {
        let file = File::open(config_path)
            .expect("Unable to read htrs_config.json");
        let config: HtrsConfig = serde_json::from_reader(file)
            .expect("Unable to read htrs_config.json");
        return config;
    }

    let mut file = File::create(config_path)
        .expect("Unable to create htrs_config.json");

    let blank_config = HtrsConfig::new();
    serde_json::to_writer_pretty(&mut file, &blank_config)
        .expect("Unable to write config to htrs_config.json");
    return blank_config;
}

fn save_config(config: HtrsConfig) {
    let config_path = Path::new("./htrs_config.json");
    let mut file = OpenOptions::new().write(true).open(config_path)
        .expect("Unable to write updated config to htrs_config.json");
    serde_json::to_writer_pretty(&mut file, &config)
        .expect("Unable to write updated config to htrs_config.json");
}
