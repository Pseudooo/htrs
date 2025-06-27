mod command_args;
mod config;
mod commands;
mod outcomes;

use crate::command_args::RootCommands::GenerateMarkdown;
use crate::command_args::{CallOutputOptions, Cli};
use crate::commands::execute_command;
use crate::config::{HtrsConfig, VersionedHtrsConfig};
use crate::outcomes::{HtrsAction, HtrsError};
use clap::Parser;
use clap_markdown::print_help_markdown;
use reqwest::blocking::{Client, Response};
use reqwest::{Method, Url};
use std::collections::HashMap;

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
            url, headers, method, display_options
        } => {
            let client = Client::new();
            let mut request_builder = client.request(method.clone(), url.clone())
                .header("User-Agent", format!("htrs/{}", env!("CARGO_PKG_VERSION")));
            for (key, value) in headers.iter() {
                request_builder = request_builder.header(key, value);
            }

            let request = match request_builder.build() {
                Ok(req) => req,
                Err(e) => return Err(HtrsError::new(&e.to_string())),
            };
            match client.execute(request) {
                Ok(res) => {
                    print_response(method, url, headers, res, display_options);
                    Ok(())
                },
                Err(e) => Err(HtrsError::new(&e.to_string())),
            }
        },
    }
}

fn print_response(method: Method, url: Url, request_headers: HashMap<String, String>, response: Response, display_options: CallOutputOptions) {
    let mut output = String::new();

    if !display_options.hide_url {
        output.push_str(&format!("{method} {url}\n"));
    }

    if !display_options.hide_request_headers {
        for (key, value) in request_headers {
            output.push_str(&format!(" ~ {key}: {value}\n"));
        }
    }

    if !display_options.hide_response_status {
        output.push_str(&format!("{}\n", response.status()));
    }

    if !display_options.hide_response_headers {
        for (key, value) in response.headers() {
            let header_str = match value.to_str() {
                Ok(s) => s,
                Err(e) => &e.to_string(),
            };
            output.push_str(&format!(" ~ {key}: {header_str}\n"));
        }
    }

    if !display_options.hide_response_body {
        let s = response.text().unwrap_or_else(|e| e.to_string());
        output.push_str(&format!("{s}\n"));
    }

    println!("{}", output);
}


