use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod htrs_config;

#[derive(Serialize, Deserialize, Clone)]
pub struct HtrsConfig {
    pub services: Vec<Service>,
    pub headers: HashMap<String, String>,
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

    pub fn get_service(&self, name: &str) -> Option<&Service> {
        for service in &self.services {
            if service.name == name || service.alias == Some(name.to_string()) {
                return Some(service);
            }
        }
        None
    }

    pub fn get_service_mut(&mut self, name: &str) -> Option<&mut Service> {
        for service in &mut self.services {
            if service.name == name || service.alias == Some(name.to_string()) {
                return Some(service);
            }
        }
        None
    }

    pub fn get_header_value(&self, header_name: String) -> Option<String> {
        self.headers.get(&header_name).cloned()
    }

    pub fn set_header(&mut self, header_name: String, header_value: String) {
    self.headers.insert(header_name, header_value);
    }

    pub fn clear_header(&mut self, header_name: String) {
        self.headers.remove(&header_name);
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
        for environment in &self.environments {
            if environment.name == name || environment.alias == Some(name.to_string()) {
                return Some(environment);
            }
        }
        None
    }

    pub fn get_environment_mut(&mut self, name: &str) -> Option<&mut Environment> {
        for environment in &mut self.environments {
            if environment.name == name || environment.alias == Some(name.to_string()) {
                return Some(environment);
            }
        }
        None
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
        return init_len != self.environments.len();
    }

    pub fn get_endpoint(&self, name: &str) -> Option<&Endpoint> {
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

    pub fn set_header(&mut self, header_name: String, header_value: String) {
        self.headers.insert(header_name, header_value);
    }

    pub fn clear_header(&mut self, header_name: String) {
        self.headers.remove(&header_name);
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
        Environment { name, alias, host, default }
    }

    pub fn display_name(&self) -> String {
        match &self.alias {
            None => self.name.to_string(),
            Some(alias) => format!("{} ({})", self.name, alias),
        }
    }
}

impl Endpoint {
    pub fn pretty_print(&self) -> String {
        let mut printed = String::new();
        printed.push_str(format!("{} | {}\n", self.name, self.path_template).as_str());
        if self.query_parameters.len() > 0 {
            printed.push_str("Query Parameters:\n");
            let parameters = self.query_parameters.iter()
                .map(|param| format!("  - {param}"))
                .collect::<Vec<String>>()
                .join("\n");
            printed.push_str(format!("{parameters}\n").as_str());
        }
        printed.push_str("\n");
        printed
    }
}