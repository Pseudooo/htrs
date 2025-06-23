mod command_args;
mod htrs_config;
mod commands;

use crate::command_args::Cli;
use crate::command_args::RootCommands::GenerateMarkdown;
use crate::commands::execute_command;
use crate::htrs_config::VersionedHtrsConfig;
use clap::Parser;
use clap_markdown::print_help_markdown;
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

struct HtrsOutcome {
    config_updated: bool,
    outcome_dialogue: String,
}

impl HtrsOutcome {
    fn new(config_updated: bool, outcome_dialogue: String) -> HtrsOutcome {
        HtrsOutcome { config_updated, outcome_dialogue}
    }
}

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
        },
        Ok(outcome) => {
            let dialogue = outcome.outcome_dialogue;
            if outcome.config_updated {
                VersionedHtrsConfig::save(config, "./htrs_config.json")
            }

            println!("{}", dialogue);
        }
    }
}


