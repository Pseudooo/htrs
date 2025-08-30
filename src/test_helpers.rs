use crate::config::{HtrsConfig, ServiceConfig};
use std::collections::HashMap;

pub struct HtrsConfigBuilder {
    pub services: Vec<ServiceConfig>
}

pub struct HtrsServiceBuilder {
    pub name: Option<String>,
    pub alias: Option<String>,
}

impl HtrsConfigBuilder {
    pub fn new() -> Self {
        Self {
            services: vec![],
        }
    }

    pub fn with_service(mut self, service_builder: HtrsServiceBuilder) -> Self {
        self.services.push(service_builder.build());
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

    pub fn build(self) -> ServiceConfig {
        let Some(name) = self.name else {
            panic!("Name not specified for built service");
        };

        ServiceConfig {
            name,
            alias: self.alias,
            environments: vec![],
            endpoints: vec![],
            headers: HashMap::new(),
        }
    }
}