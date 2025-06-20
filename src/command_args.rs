use clap::{Args, Parser, Subcommand};

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
    Call(CallOpts),
}

#[derive(Subcommand)]
pub enum ServiceCommands {
    Add {
        #[arg(long, value_name = "name")]
        name: String,
    },
    #[command(alias = "rm")]
    Remove {
        #[arg(long, value_name = "name")]
        name: String
    },
    List,

    #[command(subcommand, alias = "env")]
    Environment(EnvironmentCommands),
}

#[derive(Subcommand)]
pub enum EnvironmentCommands {
    Add {
        service_name: String,
        name: String,
        host: String,
        #[arg(long, default_value = "false")]
         default: bool,
    },
    #[clap(alias = "ls")]
    List {
        service_name: String
    },
}

#[derive(Args)]
pub struct CallOpts {
    #[arg(long, value_name = "name")]
    pub service_name: String,

    #[arg(short, long, value_name = "environment")]
    pub environment: Option<String>
}
