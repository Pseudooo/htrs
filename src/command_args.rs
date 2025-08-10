pub enum RootCommands {
    Service(ServiceCommands),
    Call(CallServiceOptions),
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

pub struct CallServiceOptions {
    pub service: String,
    pub environment: Option<String>,
    pub path: Option<String>,
    pub query: Vec<String>,
    pub header: Vec<String>,
    pub method: Option<String>,
    pub display_options: CallOutputOptions,
}

pub struct CallOutputOptions {
    pub hide_url: bool,
    pub hide_request_headers: bool,
    pub hide_response_status: bool,
    pub hide_response_headers: bool,
    pub hide_response_body: bool,
}
