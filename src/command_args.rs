use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: RootCommands,
}

#[derive(Subcommand)]
pub enum RootCommands {
    #[command(subcommand, about = "Service configuration commands")]
    Service(ServiceCommands),

    #[clap(about = "Call a service")]
    Call(CallOpts),

    #[command(visible_alias = "gen")]
    GenerateMarkdown,
}

#[derive(Subcommand)]
pub enum ServiceCommands {
    #[clap(about = "Create a new service")]
    Add {
        #[arg(long, value_name = "name", help = "Unique name of service to create")]
        name: String,
    },
    #[command(visible_alias = "rm", about = "Remove a service")]
    Remove {
        #[arg(long, value_name = "name", help = "Service name to remove")]
        name: String
    },
    #[command(visible_alias = "ls", about = "List all services")]
    List,

    #[command(subcommand, visible_alias = "env", about = "Service environment configuration commands")]
    Environment(EnvironmentCommands),
}

#[derive(Subcommand)]
pub enum EnvironmentCommands {
    #[command(about = "Add a new environment to a service")]
    Add {
        #[arg(long, value_name = "service name", help = "Service name to configure")]
        service_name: String,

        #[arg(long, value_name = "environment name", help = "Unique environment name to create")]
        name: String,

        #[arg(long, value_name = "host", help = "Hostname of the given service for the environment")]
        host: String,

        #[arg(long, default_value = "false", help = "Determine if the created environment should be set as the default")]
         default: bool,
    },
    #[clap(visible_alias = "ls", about = "List all environments for service")]
    List {
        service_name: String
    },
    #[clap(visible_alias = "rm", about = "Remove a given environment from the service")]
    Remove {
        #[clap(long, help = "Service name to remove environment from")]
        service_name: String,
        #[clap(long, help = "Environment name to remove")]
        environment_name: String,
    }
}

#[derive(Args)]
pub struct CallOpts {
    #[arg(long, value_name = "service name", help = "Service to call")]
    pub service: String,

    #[arg(short, long, value_name = "environment name", help = "Environment to call")]
    pub environment: Option<String>
}
