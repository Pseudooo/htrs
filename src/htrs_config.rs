use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct HtrsConfig {
    pub services: Vec<ServiceConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub host: String,
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

    pub fn load(path: &str) -> HtrsConfig {
        let config_path = Path::new(path);
        if config_path.exists() {
            let file = File::open(config_path)
                .expect("Unable to read htrs_config.json");
            let config: HtrsConfig = serde_json::from_reader(file)
                .expect("Unable to read htrs_config.json");
            return config;
        }

        let mut file = File::create(config_path)
            .expect("Unable to create htrs_config.json");

        let blank_config = HtrsConfig::new();
        serde_json::to_writer_pretty(&mut file, &blank_config)
            .expect("Unable to write config to htrs_config.json");
        return blank_config;
    }

    pub fn save(&self, path: &str) {
        let config_path = Path::new(path);
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(config_path)
            .expect("Unable to write updated config to htrs_config.json");
        serde_json::to_writer_pretty(&mut file, self)
            .expect("Unable to write updated config to htrs_config.json");
    }
}

impl ServiceConfig {
    pub fn new(name: String, host: String) -> ServiceConfig {
        ServiceConfig { name, host }
    }
}