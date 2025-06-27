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
    Call(CallServiceOptions),

    #[command(subcommand, about = "Global configuration")]
    Config(ConfigurationCommands),

    #[command(hide = true)]
    GenerateMarkdown,
}

#[derive(Subcommand)]
pub enum ServiceCommands {
    #[clap(about = "Create a new service")]
    Add {
        #[clap(value_name = "name", help = "Unique name of service to create")]
        name: String,
    },
    #[command(visible_alias = "rm", about = "Remove a service")]
    Remove {
        #[clap(long, value_name = "name", help = "Service name to remove")]
        name: String
    },
    #[command(visible_alias = "ls", about = "List all services")]
    List,

    #[clap(about = "Service configuration")]
    Config {
        #[clap(value_name = "service name", help = "Service name to configure")]
        service_name: String,
        #[command(subcommand)]
        config_command: ConfigurationCommands,
    },

    #[command(subcommand, visible_alias = "env", about = "Service environment configuration commands")]
    Environment(EnvironmentCommands),
}

#[derive(Subcommand)]
pub enum EnvironmentCommands {
    #[command(about = "Add a new environment to a service")]
    Add {
        #[clap(value_name = "service name", help = "Service to configure")]
        service_name: String,

        #[clap(value_name = "environment name", help = "Unique environment name to create")]
        name: String,

        #[clap(value_name = "host", help = "Hostname of the for service in new environment")]
        host: String,

        #[arg(short, long, default_value = "false", help = "Is the default environment for service")]
        default: bool,
    },
    #[clap(visible_alias = "ls", about = "List all environments for service")]
    List {
        service_name: String
    },
    #[clap(visible_alias = "rm", about = "Remove an environment from the service")]
    Remove {
        #[clap(help = "Service to remove environment from")]
        service_name: String,
        #[clap(help = "Environment to remove")]
        environment_name: String,
    }
}

#[derive(Subcommand)]
pub enum ConfigurationCommands {
    #[command(subcommand, about = "Configure headers")]
    Header(HeaderCommands),
}

#[derive(Subcommand)]
pub enum HeaderCommands {
    #[command(about = "Set a header value")]
    Set {
        #[clap(value_name = "header", help = "Header name")]
        header: String,
        #[clap(value_name = "header_value", help = "Header value")]
        value: String,
    },
    #[command(about = "Clear a header value")]
    Clear {
        #[clap(value_name = "header", help = "Header name to clear")]
        header: String,
    },
}

#[derive(Args)]
pub struct CallServiceOptions {
    pub service: String,

    #[arg(short, long, value_name = "environment", help = "Environment to call")]
    pub environment: Option<String>,

    #[arg(short, long, value_name = "path", help = "Path to call for host")]
    pub path: Option<String>,

    #[arg(short, long, value_name = "query", help = "Query string key=value pair")]
    pub query: Vec<String>,

    #[arg(long, value_name = "header", help = "Header values as key=value pairs")]
    pub header: Vec<String>,
    
    #[arg(long, value_name = "method", help = "The HTTP Method to use when making call, i.e. GET or POST")]
    pub method: Option<String>,

    #[clap(flatten)]
    pub display_options: CallOutputOptions,
}

#[derive(Args)]
pub struct CallOutputOptions {
    #[clap(help = "Hide the requested url")]
    pub hide_url: bool,
    #[clap(help = "Hide the request headers")]
    pub hide_request_headers: bool,
    #[clap(help = "Hide the response status code")]
    pub hide_response_status: bool,
    #[clap(help = "Hide the response headers")]
    pub hide_response_headers: bool,
    #[clap(help = "Hide the response body")]
    pub hide_response_body: bool,
}
