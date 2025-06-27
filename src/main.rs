mod command_args;
mod config;
mod commands;
mod outcomes;

use crate::command_args::Cli;
use crate::command_args::RootCommands::GenerateMarkdown;
use crate::commands::execute_command;
use crate::config::{HtrsConfig, VersionedHtrsConfig};
use crate::outcomes::{HtrsAction, HtrsError};
use clap::Parser;
use clap_markdown::print_help_markdown;
use reqwest::blocking::Client;

fn main() {
    let parsed_args = Cli::parse();
    if let GenerateMarkdown = parsed_args.command {
        print_help_markdown::<Cli>();
        return;
    }

    let mut config = VersionedHtrsConfig::load();
    let cmd_result = execute_command(&mut config, parsed_args.command);
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
            url, headers, method
        } => {
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
                Ok(res) => {
                    println!("Received {} response", res.status());
                    Ok(())
                },
                Err(e) => Err(HtrsError::new(&e.to_string())),
            }
        },
    }
}


