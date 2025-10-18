use crate::config::{Endpoint, Environment, HtrsConfig, Service};
use std::collections::HashMap;

pub struct HtrsConfigBuilder {
    pub services: Vec<Service>,
    pub headers: HashMap<String,String>,
}

pub struct HtrsServiceBuilder {
    pub name: Option<String>,
    pub alias: Option<String>,
    pub environments: Vec<Environment>,
    pub endpoints: Vec<Endpoint>,
}

impl HtrsConfigBuilder {
    pub fn new() -> Self {
        Self {
            services: vec![],
            headers: HashMap::new(),
        }
    }

    pub fn with_service(mut self, service_builder: HtrsServiceBuilder) -> Self {
        self.services.push(service_builder.build());
        self
    }

    pub fn with_header(mut self, header_name: &str, header_value: &str) -> Self {
        self.headers.insert(header_name.to_string(), header_value.to_string());
        self
    }

    pub fn build(self) -> HtrsConfig {
        HtrsConfig {
            services: self.services,
            headers: HashMap::new(),
        }
    }
}

impl HtrsServiceBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            alias: None,
            environments: vec![],
            endpoints: vec![],
        }
    }

    pub fn with_name(mut self, name: &str) -> HtrsServiceBuilder {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_alias(mut self, alias: &str) -> HtrsServiceBuilder {
        self.alias = Some(alias.to_string());
        self
    }

    pub fn with_environment(mut self, name: &str, alias: Option<&str>, host: &str, default: bool) -> HtrsServiceBuilder {
        self.environments.push(Environment {
            name: name.to_string(),
            alias: match alias {
                Some(alias) => Some(alias.to_string()),
                _ => None,
            },
            host: host.to_string(),
            default,
            headers: HashMap::new(),
        });
        self
    }

    pub fn with_endpoint(mut self, name: &str, path: &str, parameters: Vec<&str>) -> HtrsServiceBuilder {
        self.endpoints.push(Endpoint {
            name: name.to_string(),
            path_template: path.to_string(),
            query_parameters: parameters.iter().map(|p| p.to_string()).collect(),
        });
        self
    }

    pub fn build(self) -> Service {
        let Some(name) = self.name else {
            panic!("Name not specified for built service");
        };

        Service {
            name,
            alias: self.alias,
            environments: self.environments,
            endpoints: self.endpoints,
            headers: HashMap::new(),
        }
    }
}