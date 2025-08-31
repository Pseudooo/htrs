use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum VersionedHtrsConfig {
    V0_0_1(HtrsConfig),
}

impl VersionedHtrsConfig {
    /// Generate the path to the configration file, using the directory
    /// of the executable as the base path.
    fn config_path() -> PathBuf {
        std::env::current_exe()
            .expect("Unable to get executable location")
            .parent()
            .expect("Unable to get parent directory")
            .join("config.json")
    }

    pub fn load() -> HtrsConfig {
        let config_path = Self::config_path();
        if config_path.exists() {
            let file = File::open(config_path).expect("Unable to read config.json");
            let VersionedHtrsConfig::V0_0_1(config): VersionedHtrsConfig =
                serde_json::from_reader(file).expect("Unable to read config.json");
            return config;
        }

        let mut file = File::create(config_path)
            .expect("Unable to create config.json");

        let blank_config = HtrsConfig::new();
        let blank_versioned_config = VersionedHtrsConfig::V0_0_1(blank_config.clone());
        serde_json::to_writer_pretty(&mut file, &blank_versioned_config)
            .expect("Unable to write config to config.json");
        blank_config
    }

    pub fn save(config: HtrsConfig) {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(VersionedHtrsConfig::config_path())
            .expect("Unable to write updated config to config.json");
        let versioned_config = VersionedHtrsConfig::V0_0_1(config);
        serde_json::to_writer_pretty(&mut file, &versioned_config)
            .expect("Unable to write updated config to config.json");
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HtrsConfig {
    pub services: Vec<ServiceConfig>,
    pub headers: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ServiceConfig {
    pub name: String,
    pub alias: Option<String>,
    pub environments: Vec<ServiceEnvironmentConfig>,
    pub headers: HashMap<String, String>,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ServiceEnvironmentConfig {
    pub name: String,
    pub alias: Option<String>,
    pub host: String,
    pub default: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Endpoint {
    pub name: String,
    pub path_template: String,
    pub query_parameters: Vec<String>,
}

impl HtrsConfig {
    pub fn new() -> HtrsConfig {
        HtrsConfig { services: Vec::new(), headers: HashMap::new() }
    }

    pub fn remove_service(&mut self, name: &str) -> bool {
        let init_length = self.services.len();
        self.services.retain(|service| service.name != name && service.alias != Some(name.to_string()));
        return init_length != self.services.len();
    }

    pub fn get_service(&self, name: &str) -> Option<&ServiceConfig> {
        for service in &self.services {
            if service.name == name || service.alias == Some(name.to_string()) {
                return Some(service);
            }
        }
        None
    }

    pub fn get_service_mut(&mut self, name: &str) -> Option<&mut ServiceConfig> {
        for service in &mut self.services {
            if service.name == name || service.alias == Some(name.to_string()) {
                return Some(service);
            }
        }
        None
    }
}

impl ServiceConfig {
    pub fn new(name: String, alias: Option<String>) -> ServiceConfig {
        ServiceConfig {
            name,
            alias,
            environments: vec![],
            headers: HashMap::new(),
            endpoints: vec![]
        }
    }

    pub fn get_environment(&self, name: &str) -> Option<&ServiceEnvironmentConfig> {
        for environment in &self.environments {
            if environment.name == name || environment.alias == Some(name.to_string()) {
                return Some(environment);
            }
        }
        None
    }

    pub fn get_default_environment(&self) -> Option<&ServiceEnvironmentConfig> {
        for environment in &self.environments {
            if environment.default {
                return Some(environment)
            }
        }
        None
    }

    pub fn get_default_environment_mut(&mut self) -> Option<&mut ServiceEnvironmentConfig> {
        for environment in &mut self.environments {
            if environment.default {
                return Some(environment)
            }
        }
        None
    }

    pub fn remove_environment(&mut self, name: &str) -> bool {
        let init_len = self.environments.len();
        self.environments.retain(|x| x.name != name && x.alias != Some(name.to_string()));
        return init_len != self.environments.len();
    }

    pub fn endpoint_exists(&self, name: &str) -> bool {
        for endpoint in &self.endpoints {
            if endpoint.name == name {
                return true;
            }
        }
        false
    }

    pub fn find_endpoint(&self, name: &str) -> Option<&Endpoint> {
        for endpoint in &self.endpoints {
            if endpoint.name == name {
                return Some(endpoint);
            }
        }
        None
    }

    pub fn remove_endpoint(&mut self, name: &str) -> bool {
        let init_len = self.endpoints.len();
        self.endpoints.retain(|x| x.name != name);
        return init_len != self.endpoints.len();
    }
}

impl ServiceEnvironmentConfig {
    pub fn new(name: String, alias: Option<String>, host: String, default: bool) -> ServiceEnvironmentConfig {
        ServiceEnvironmentConfig { name, alias, host, default }
    }
}