use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum VersionedHtrsConfig {
    V0_0_1(HtrsConfig),
}

impl VersionedHtrsConfig {
    /// Generate the path to the configration file, using the directory
    /// of the executable as the base path.
    pub fn config_path() -> PathBuf {
        std::env::current_exe()
            .expect("Unable to get executable location")
            .parent()
            .expect("Unable to get parent directory")
            .join("htrs_config.json")
    }

    pub fn load(config_path: &Path) -> VersionedHtrsConfig {
        if config_path.exists() {
            let file = File::open(config_path).expect("Unable to read htrs_config.json");
            let config: VersionedHtrsConfig =
                serde_json::from_reader(file).expect("Unable to read htrs_config.json");
            return config;
        }

        let mut file = File::create(config_path)
            .expect("Unable to create htrs_config.json");

        let blank_config = VersionedHtrsConfig::V0_0_1(HtrsConfig::new());
        serde_json::to_writer_pretty(&mut file, &blank_config)
            .expect("Unable to write config to htrs_config.json");
        return blank_config;
    }

    pub fn save(config: HtrsConfig, config_path: &Path) {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(config_path)
            .expect("Unable to write updated config to htrs_config.json");
        let versioned_config = VersionedHtrsConfig::V0_0_1(config);
        serde_json::to_writer_pretty(&mut file, &versioned_config)
            .expect("Unable to write updated config to htrs_config.json");
    }
}

#[derive(Serialize, Deserialize)]
pub struct HtrsConfig {
    pub services: Vec<ServiceConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub environments: Vec<ServiceEnvironmentConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceEnvironmentConfig {
    pub name: String,
    pub host: String,
    pub default: bool,
}

impl HtrsConfig {
    pub fn new() -> HtrsConfig {
        HtrsConfig { services: Vec::new() }
    }

    pub fn service_defined(&self, name: &str) -> bool {
        for service in &self.services {
            if service.name == name {
                return true;
            }
        }
        return false;
    }

    pub fn find_service_config(&self, name: &str) -> Option<&ServiceConfig> {
        for service in &self.services {
            if service.name == name {
                return Some(service);
            }
        }
        None
    }

    pub fn find_service_config_mut(&mut self, name: &str) -> Option<&mut ServiceConfig> {
        for service in &mut self.services {
            if service.name == name {
                return Some(service);
            }
        }
        None
    }
}

impl ServiceConfig {
    pub fn new(name: String) -> ServiceConfig {
        ServiceConfig { name, environments: vec![] }
    }

    pub fn environment_exists(&self, name: &str) -> bool {
        for environment in &self.environments {
            if environment.name == name {
                return true;
            }
        }
        return false;
    }

    pub fn find_environment(&self, name: &str) -> Option<&ServiceEnvironmentConfig> {
        for environment in &self.environments {
            if environment.name == name {
                return Some(environment);
            }
        }
        None
    }

    pub fn find_default_environment(&self) -> Option<&ServiceEnvironmentConfig> {
        for environment in &self.environments {
            if environment.default {
                return Some(environment);
            }
        }
        None
    }
    
    pub fn find_default_environment_mut(&mut self) -> Option<&mut ServiceEnvironmentConfig> {
        for environment in &mut self.environments {
            if environment.default {
                return Some(environment);
            }
        }
        None
    }
    
    pub fn remove_environment(&mut self, name: &str) -> bool {
        let init_len = self.environments.len();
        self.environments.retain(|x| x.name != name);
        return init_len != self.environments.len();
    }
}

impl ServiceEnvironmentConfig {
    pub fn new(name: String, host: String, default: bool) -> ServiceEnvironmentConfig {
        ServiceEnvironmentConfig { name, host, default }
    }
}