use crate::commands::call_command::CallServiceEndpointCommand;
use crate::commands::service_commands::ServiceCommand;

pub enum RootCommands {
    Service(ServiceCommand),
    Call(CallServiceEndpointCommand),
    Config(ConfigurationCommands),
}

pub enum ConfigurationCommands {
    Header(HeaderCommands),
}

pub enum HeaderCommands {
    Set {
        header: String,
        value: String,
    },
    Clear {
        header: String,
    },
}
