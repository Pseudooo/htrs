mod command_args;
mod htrs_config;
mod commands;
mod outcomes;

use crate::command_args::Cli;
use crate::command_args::RootCommands::GenerateMarkdown;
use crate::commands::execute_command;
use crate::htrs_config::VersionedHtrsConfig;
use clap::Parser;
use clap_markdown::print_help_markdown;

fn main() {
    let parsed_args = Cli::parse();
    if let GenerateMarkdown = parsed_args.command {
        print_help_markdown::<Cli>();
        return;
    }

    let versioned_config = VersionedHtrsConfig::load("./htrs_config.json");
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


