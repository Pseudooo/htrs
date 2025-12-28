use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::path::PathBuf;

impl HtrsConfig {
    /// Generate the path to the configration file, using the directory
    /// of the executable as the base path.
    fn config_path() -> Result<PathBuf, String> {
        if let Ok(path) = std::env::var("HTRS_CONFIG_PATH") {
            return Ok(PathBuf::from(path));
        }

        let exe_path = match std::env::current_exe() {
            Ok(path) => path,
            Err(e) => return Err(e.to_string())
        };

        let directory = match exe_path.parent() {
            Some(path) => path,
            None => return Err(format!("No parent directory could be found for path `{}`", exe_path.display())),
        };

        Ok(directory.join("config.json"))
    }

    pub fn load() -> Result<HtrsConfig, String> {
        let config_path = Self::config_path()?;
        if !config_path.exists() {
            return Ok(HtrsConfig::new());
        }

        let handle = OpenOptions::new()
            .read(true)
            .open(config_path)
            .expect("Unable to open config file");

        match serde_json::from_reader(handle) {
            Ok(config) => Ok(config),
            Err(e) => Err(format!("Unable to read config json: {e}")),
        }
    }

    pub fn save(self) -> Result<(), String> {
        let config_path = Self::config_path()?;

        let mut file = match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(config_path) {
            Ok(f) => f,
            Err(e) => return Err(format!("Failed to open config file: {e}"))
        };

        match serde_json::to_writer_pretty(&mut file, &self) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to write config json to file: {e}"))
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HtrsConfig {
    pub services: Vec<Service>,
    pub headers: HashMap<String, String>,
    pub presets: Vec<Preset>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Preset {
    pub name: String,
    pub values: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Service {
    pub name: String,
    pub alias: Option<String>,
    pub environments: Vec<Environment>,
    pub headers: HashMap<String, String>,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Environment {
    pub name: String,
    pub alias: Option<String>,
    pub host: String,
    pub default: bool,
    pub headers: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Endpoint {
    pub name: String,
    pub path_template: String,
    pub query_parameters: Vec<QueryParameter>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct QueryParameter {
    pub name: String,
    pub required: bool,
}

impl QueryParameter {
    pub fn from_shorthand(query_param: &str) -> QueryParameter {
        match query_param.starts_with('*') {
            true => QueryParameter {
                name: query_param[1..].to_string(),
                required: true,
            },
            false => QueryParameter {
                name: query_param.to_string(),
                required: false,
            }
        }
    }
}

impl HtrsConfig {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
            headers: HashMap::new(),
            presets: Vec::new(),
        }
    }

    pub fn remove_service(&mut self, name: &str) -> bool {
        let init_length = self.services.len();
        self.services.retain(|service| service.name != name && service.alias != Some(name.to_string()));
        init_length != self.services.len()
    }

    pub fn get_service(&self, name: &str) -> Option<&Service> {
        self.services.iter().find(|&service| service.name == name || service.alias == Some(name.to_string())).map(|v| v as _)
    }

    pub fn get_service_mut(&mut self, name: &str) -> Option<&mut Service> {
        self.services.iter_mut().find(|s| s.name == name)
    }

    pub fn get_preset(&self, name: &str) -> Option<&Preset> {
        self.presets.iter().find(|p| p.name == name)
    }

    pub fn get_preset_mut(&mut self, name: &str) -> Option<&mut Preset> {
        self.presets.iter_mut().find(|p| p.name == name)
    }

    pub fn remove_preset(&mut self, name: &str) -> bool {
        let init_length = self.presets.len();
        self.presets.retain(|preset| preset.name != name);
        init_length != self.presets.len()
    }
}

impl Service {
    pub fn new(name: String, alias: Option<String>) -> Service {
        Service {
            name,
            alias,
            environments: vec![],
            headers: HashMap::new(),
            endpoints: vec![]
        }
    }

    pub fn get_environment(&self, name: &str) -> Option<&Environment> {
        self.environments.iter().find(|&environment| environment.name == name || environment.alias == Some(name.to_string()))
    }

    pub fn get_environment_mut(&mut self, name: &str) -> Option<&mut Environment> {
        self.environments.iter_mut().find(|e| e.name == name || e.alias == Some(name.to_string()))
    }

    pub fn get_default_environment(&self) -> Option<&Environment> {
        for environment in &self.environments {
            if environment.default {
                return Some(environment)
            }
        }
        None
    }

    pub fn get_default_environment_mut(&mut self) -> Option<&mut Environment> {
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
        init_len != self.environments.len()
    }

    pub fn get_endpoint(&self, name: &str) -> Option<&Endpoint> {
        self.endpoints.iter().find(|&endpoint| endpoint.name == name)
    }
    
    pub fn get_endpoint_mut(&mut self, name: &str) -> Option<&mut Endpoint> {
        self.endpoints.iter_mut().find(|e| e.name == name)
    }

    pub fn remove_endpoint(&mut self, name: &str) -> bool {
        let init_len = self.endpoints.len();
        self.endpoints.retain(|x| x.name != name);
        init_len != self.endpoints.len()
    }

    pub fn display_name(&self) -> String {
        match &self.alias {
            None => self.name.to_string(),
            Some(alias) => format!("{} ({})", self.name, alias),
        }
    }
}

impl Environment {
    pub fn new(name: String, alias: Option<String>, host: String, default: bool) -> Environment {
        Environment {
            name,
            alias,
            host,
            default,
            headers: HashMap::new(),
        }
    }

    pub fn display_name(&self) -> String {
        match &self.alias {
            None => self.name.to_string(),
            Some(alias) => format!("{} ({})", self.name, alias),
        }
    }
}

pub trait HeaderItem {
    fn set_header(&mut self, header_name: String, header_value: String);
    fn clear_header(&mut self, header_name: String);
}

impl HeaderItem for HtrsConfig {
    fn set_header(&mut self, header_name: String, header_value: String) {
        self.headers.insert(header_name, header_value);
    }

    fn clear_header(&mut self, header_name: String) {
        self.headers.remove(&header_name);
    }
}

impl HeaderItem for Service {
    fn set_header(&mut self, header_name: String, header_value: String) {
        self.headers.insert(header_name, header_value);
    }

    fn clear_header(&mut self, header_name: String) {
        self.headers.remove(&header_name);
    }
}

impl HeaderItem for Environment {
    fn set_header(&mut self, header_name: String, header_value: String) {
        self.headers.insert(header_name, header_value);
    }

    fn clear_header(&mut self, header_name: String) {
        self.headers.remove(&header_name);
    }
}