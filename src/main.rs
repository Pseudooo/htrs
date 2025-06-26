mod command_args;
mod htrs_config;
mod commands;
mod outcomes;

use crate::command_args::Cli;
use crate::command_args::RootCommands::GenerateMarkdown;
use crate::commands::execute_command;
use crate::htrs_config::VersionedHtrsConfig;
use crate::outcomes::{HtrsAction, HtrsError};
use clap::Parser;
use clap_markdown::print_help_markdown;
use reqwest::blocking::{Client, Response};
use HtrsAction::MakeRequest;

fn main() {
    let parsed_args = Cli::parse();
    if let GenerateMarkdown = parsed_args.command {
        print_help_markdown::<Cli>();
        return;
    }

    let config_path = VersionedHtrsConfig::config_path();
    let versioned_config = VersionedHtrsConfig::load(&config_path);
    let mut config = match versioned_config {
        VersionedHtrsConfig::V0_0_1(config) => config,
    };

    let result = execute_command(&mut config, parsed_args.command);
    match result {
        Err(e) => {
            println!("{}", e.details);
            return;
        }
        Ok(outcome) => {
            let dialogue = outcome.outcome_dialogue;
            if outcome.config_updated {
                VersionedHtrsConfig::save(config, &config_path)
            }

            if let Some(action) = outcome.action {
                match handle_outcome_action(action) {
                    Ok(response) => println!("Received {} response", response.status()),
                    Err(e) => println!("Failed to call: {e}"),
                }
            }

            if let Some(dialogue) = dialogue {
                println!("{}", dialogue);
            }
        }
    }
}

fn handle_outcome_action(action: HtrsAction) -> Result<Response, HtrsError> {
    let MakeRequest { url, headers, method } = action;
    let client = Client::new();
    let mut request_builder = client.request(method, url)
        .header("User-Agent", format!("htrs/{}", env!("CARGO_PKG_VERSION")));
    for (key, value) in headers {
        request_builder = request_builder.header(key.as_str(), value.as_str());
    }

    let request = match request_builder.build() {
        Ok(req) => req,
        Err(e) => return Err(HtrsError::new(&e.to_string())),
    };
    match client.execute(request) {
        Ok(res) => Ok(res),
        Err(e) => Err(HtrsError::new(&e.to_string())),
    }
}


