use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
