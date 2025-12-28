pub mod v1config {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize, Clone)]
    pub struct HtrsConfigV1 {
        pub services: Vec<ServiceV1>,
        pub headers: HashMap<String, String>,
        pub presets: Vec<PresetV1>
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct PresetV1 {
        pub name: String,
        pub values: HashMap<String, String>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct ServiceV1 {
        pub name: String,
        pub alias: Option<String>,
        pub environments: Vec<EnvironmentV1>,
        pub headers: HashMap<String, String>,
        pub endpoints: Vec<EndpointV1>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct EnvironmentV1 {
        pub name: String,
        pub alias: Option<String>,
        pub host: String,
        pub default: bool,
        pub headers: HashMap<String, String>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct EndpointV1 {
        pub name: String,
        pub path_template: String,
        pub query_parameters: Vec<QueryParameterV1>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct QueryParameterV1 {
        pub name: String,
        pub required: bool,
    }

}