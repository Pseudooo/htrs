use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: RootCommands
}

#[derive(Subcommand)]
pub enum RootCommands {
    #[command(subcommand)]
    Service(ServiceCommands),
    Call,
}

#[derive(Subcommand)]
pub enum ServiceCommands {
    Add {
        #[arg(long, value_name = "name")]
        name: String,

        #[arg(long, value_name = "url")]
        host: String,
    },
    List,
}