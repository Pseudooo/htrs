use crate::common::config::{Environment, HtrsConfig, Service};
use std::collections::HashMap;
use std::fs::{remove_file, OpenOptions};
use std::path::PathBuf;

#[path = "../../src/config.rs"]
mod config;

pub fn get_config_path() -> PathBuf {
    std::env::current_exe()
        .expect("Unable to get executable location")
        .parent()
        .expect("Unable to get parent directory")
        .parent()
        .expect("Unable to get parent directory")
        .join("config.json")
}

pub fn setup(init_config: Option<HtrsConfig>) {
    let path = get_config_path();
    if path.exists() {
        remove_file(path.clone()).expect("Failed to clear existing config file");
    }

    if let Some(init_config) = init_config {
        let handle = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .unwrap();

        serde_json::to_writer_pretty(handle, &init_config).unwrap();
    }
}

pub fn get_config() -> HtrsConfig {
   serde_json::from_reader(std::fs::File::open(get_config_path()).unwrap()).unwrap()
}

pub struct HtrsConfigBuilder {
    pub services: Vec<Service>,
}

pub struct ServiceBuilder {
    pub name: Option<String>,
    pub alias: Option<String>,
    pub environments: Vec<Environment>,
}

pub struct EnvironmentBuilder {
    pub name: Option<String>,
    pub alias: Option<String>,
    pub host: Option<String>,
    pub default: bool,
}

impl HtrsConfigBuilder {
    pub fn new() -> Self {
        Self {
            services: vec![],
        }
    }

    pub fn with_service(mut self, builder: ServiceBuilder) -> Self {
        self.services.push(builder.build());
        self
    }

    pub fn build(self) -> HtrsConfig {
        HtrsConfig {
            services: self.services,
            headers: HashMap::new(),
        }
    }
}

impl ServiceBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            alias: None,
            environments: vec![]
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_alias(mut self, alias: &str) -> Self {
        self.alias = Some(alias.to_string());
        self
    }

    pub fn with_environment(mut self, builder: EnvironmentBuilder) -> Self {
        self.environments.push(builder.build());
        self
    }

    pub fn build(self) -> Service {
        Service {
            name: self.name.unwrap(),
            alias: self.alias,
            headers: HashMap::new(),
            endpoints: vec![],
            environments: self.environments,
        }
    }
}

impl EnvironmentBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            alias: None,
            host: None,
            default: false,
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_alias(mut self, alias: &str) -> Self {
        self.alias = Some(alias.to_string());
        self
    }

    pub fn with_host(mut self, host: &str) -> Self {
        self.host = Some(host.to_string());
        self
    }

    pub fn with_default(mut self) -> Self {
        self.default = true;
        self
    }

    pub fn build(self) -> Environment {
        Environment {
            name: self.name.unwrap(),
            alias: self.alias,
            host: self.host.unwrap(),
            default: self.default,
        }
    }
}