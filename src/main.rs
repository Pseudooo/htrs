use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: CliCommands
}

#[derive(Subcommand)]
enum CliCommands {
    #[command(alias = "c")]
    Call,
    #[command(alias = "cfg")]
    Config,
}

fn main() {

    match Cli::try_parse() {
        Ok(x) => {
            exec(x);
        }
        Err(e) => {
            e.exit();
        }
    }
}

fn exec(arg: Cli) {
    match arg.command {
        CliCommands::Call => {
            println!("Using Call")
        },
        CliCommands::Config => {
            println!("Using config");
        }
    }
}
