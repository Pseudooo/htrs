use crate::commands::call_command::CallServiceEndpointCommand;
use crate::commands::service_commands::ServiceCommand;

pub enum RootCommands {
    Service(ServiceCommand),
    Call(CallServiceEndpointCommand),
    Config(ConfigurationCommands),
}

pub enum ServiceCommands {
    Add {
        name: String,
    },
    Remove {
        name: String
    },
    List,
    Config {
        service_name: String,
        config_command: ConfigurationCommands,
    },
    Environment(EnvironmentCommands),
    Endpoint {
        service_name: String,
        command: EndpointCommands
    }
}

pub enum EnvironmentCommands {
    Add {
        service_name: String,
        name: String,
        host: String,
        default: bool,
    },
    List {
        service_name: String
    },
    Remove {
        service_name: String,
        environment_name: String,
    }
}

pub enum EndpointCommands {
    Add {
        name: String,
        path_template: String,
        query_parameters: Vec<String>,
    },
    List,
    Remove {
        name: String,
    }
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
