mod command_args;
mod config;
mod commands;
mod outcomes;
mod command_builder;

#[cfg(test)]
mod test_helpers;

use crate::command_args::RootCommands;
use crate::command_builder::get_root_command;
use crate::commands::execute_command;
use crate::config::{HtrsConfig, VersionedHtrsConfig};
use crate::outcomes::{HtrsAction, HtrsError};
use reqwest::blocking::Client;
use reqwest::{Method, Url};
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
            url: base_url, query_parameters, method, headers, show_body
        } => {
            let url = apply_query_params_to_url(base_url, query_parameters)?;
            execute_request(method, url, headers, show_body)
        },
    }
}

fn apply_query_params_to_url(base_url: Url, query_params: HashMap<String, String>) -> Result<Url, HtrsError> {
    if query_params.len() == 0 {
        return Ok(base_url);
    }

    let query_params_str = query_params.iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<String>>()
        .join("&");

    match base_url.join(&format!("?{query_params_str}")) {
        Ok(url) => Ok(url),
        Err(e) => Err(HtrsError::new(&format!("Failed to build url with query parameters: {e}"))),
    }
}

fn execute_request(method: Method, url: Url, headers: HashMap<String, String>, show_body: bool) -> Result<(), HtrsError> {
    let mut req_headers = get_default_headers();
    merge_hashmaps(&mut req_headers, &headers);

    let client = Client::new();
    let mut request_builder = client.request(method.clone(), url.clone());
    for (k, v) in req_headers {
        request_builder = request_builder.header(k, v);
    }

    let request = match request_builder.build() {
        Ok(request) => request,
        Err(e) => return Err(HtrsError::new(&e.to_string())),
    };

    match client.execute(request) {
        Ok(response) => {
            if !show_body {
                println!("{} | {} | {}", response.status(), &method, url);
            }

            let response_text = response.text()
                .unwrap_or_else(|e| format!("<Failed to read response body: {}>", e));
            println!("{}", response_text);
            Ok(())
        },
        Err(e) => {
            Err(HtrsError::new(&e.to_string()))
        }
    }
}

fn merge_hashmaps(into: &mut HashMap<String, String>, from: &HashMap<String, String>) {
    for (k, v) in from {
        into.insert(k.clone(), v.clone());
    }
}

fn get_default_headers() -> HashMap<String, String> {
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("User-Agent".to_string(), format!("htrs/{}", env!("CARGO_PKG_VERSION")));
    headers
}