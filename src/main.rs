mod command_args;
mod config;
mod commands;
mod outcomes;
mod command_builder;

use crate::command_args::RootCommands;
use crate::command_builder::get_root_command;
use crate::commands::execute_command;
use crate::config::{HtrsConfig, VersionedHtrsConfig};
use crate::outcomes::{HtrsAction, HtrsError};
use reqwest::blocking::Client;
use reqwest::Url;
use std::collections::HashMap;

fn main() {
    let mut config = VersionedHtrsConfig::load();

    let command_matches = get_root_command(&config).get_matches();
    let command = RootCommands::bind_from_matches(&config, &command_matches);

    let cmd_result = execute_command(&mut config, command);
    let exec_result = match cmd_result {
        Err(e) => {
            println!("{}", e.details);
            return;
        }
        Ok(action) => handle_action(action, config)
    };

    if let Err(e) = exec_result {
        println!("{}", e.details);
    }
}

fn handle_action(action: HtrsAction, config: HtrsConfig) -> Result<(), HtrsError>{
    match action {
        HtrsAction::PrintDialogue(dialogue) => {
            println!("{}", dialogue);
            Ok(())
        },
        HtrsAction::UpdateConfig => {
            VersionedHtrsConfig::save(config);
            Ok(())
        },
        HtrsAction::MakeRequest {
            url: base_url, query_parameters, method
        } => {
            let client = Client::new();

            let url = apply_query_params_to_url(base_url, query_parameters)?;
            let request_builder = client.request(method.clone(), url.clone());

            let request = match request_builder.build() {
                Ok(req) => req,
                Err(e) => return Err(HtrsError::new(&e.to_string())),
            };
            match client.execute(request) {
                Ok(res) => {
                    println!("Received {}", res.status());
                    Ok(())
                },
                Err(e) => Err(HtrsError::new(&e.to_string())),
            }
        },
    }
}

fn apply_query_params_to_url(base_url: Url, query_params: HashMap<String, String>) -> Result<Url, HtrsError> {
    let query_params_str = query_params.iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<String>>()
        .join("&");

    match base_url.join(&format!("?{query_params_str}")) {
        Ok(url) => Ok(url),
        Err(e) => Err(HtrsError::new(&format!("Failed to build url with query parameters: {e}"))),
    }
}
