use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HtrsConfigV0_0_1 {
    pub services: Vec<ServiceConfigV0_0_1>,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceConfigV0_0_1 {
    pub name: String,
    pub environments: Vec<ServiceEnvironmentConfigV0_0_1>,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceEnvironmentConfigV0_0_1 {
    pub name: String,
    pub host: String,
    pub default: bool,
}