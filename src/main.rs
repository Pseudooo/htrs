mod command_args;
mod htrs_config;
mod commands;

use crate::command_args::RootCommands::GenerateMarkdown;
use crate::command_args::Cli;
use crate::commands::execute_command;
use crate::htrs_config::HtrsConfig;
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

struct HtrsOutcome<'a> {
    config: &'a HtrsConfig,
    config_updated: bool,
    outcome_dialogue: String,
}

impl<'a> HtrsOutcome<'a> {
    fn new(config: &'a HtrsConfig, config_updated: bool, outcome_dialogue: String) -> HtrsOutcome<'a> {
        HtrsOutcome { config, config_updated, outcome_dialogue}
    }
}

fn main() {
    let parsed_args = Cli::parse();
    if let GenerateMarkdown = parsed_args.command {
        print_help_markdown::<Cli>();
        return;
    }

    let mut config = HtrsConfig::load("./htrs_config.json");

    let result = execute_command(&mut config, parsed_args.command);
    match result {
        Err(e) => {
            println!("{}", e.details);
            return;
        },
        Ok(outcome) => {
            let dialogue = outcome.outcome_dialogue;
            if outcome.config_updated {
                outcome.config.save("htrs_config.json");
            }

            println!("{}", dialogue);
        }
    }
}


