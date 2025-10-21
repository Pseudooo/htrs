#[path = "../../src/config.rs"]
mod config;

#[cfg(test)]
pub mod test_helpers {
    use crate::common::config::{Endpoint, Environment, HtrsConfig, Service};
    use std::collections::HashMap;
    use std::fs::{remove_file, OpenOptions};
    use std::path::PathBuf;

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
        pub headers: HashMap<String, String>,
    }

    pub struct ServiceBuilder {
        pub name: Option<String>,
        pub alias: Option<String>,
        pub endpoints: Vec<Endpoint>,
        pub environments: Vec<Environment>,
        pub headers: HashMap<String, String>,
    }

    pub struct EndpointBuilder {
        pub name: Option<String>,
        pub path: Option<String>,
        pub query_params: Vec<String>,
    }

    pub struct EnvironmentBuilder {
        pub name: Option<String>,
        pub alias: Option<String>,
        pub host: Option<String>,
        pub default: bool,
        pub headers: HashMap<String, String>
    }

    impl HtrsConfigBuilder {
        pub fn new() -> Self {
            Self {
                services: vec![],
                headers: HashMap::new(),
            }
        }

        pub fn with_service(mut self, builder: ServiceBuilder) -> Self {
            self.services.push(builder.build());
            self
        }

        pub fn with_header(mut self, name: &str, value: &str) -> Self {
            self.headers.insert(name.to_string(), value.to_string());
            self
        }

        pub fn build(self) -> HtrsConfig {
            HtrsConfig {
                services: self.services,
                headers: self.headers,
            }
        }
    }

    impl ServiceBuilder {
        pub fn new() -> Self {
            Self {
                name: None,
                alias: None,
                endpoints: vec![],
                environments: vec![],
                headers: HashMap::new(),
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

        pub fn with_endpoint(mut self, builder: EndpointBuilder) -> Self {
            self.endpoints.push(builder.build());
            self
        }

        pub fn with_environment(mut self, builder: EnvironmentBuilder) -> Self {
            self.environments.push(builder.build());
            self
        }

        pub fn with_header(mut self, name: &str, value: &str) -> Self {
            self.headers.insert(name.to_string(), value.to_string());
            self
        }

        pub fn build(self) -> Service {
            Service {
                name: self.name.unwrap(),
                alias: self.alias,
                headers: self.headers,
                endpoints: self.endpoints,
                environments: self.environments,
            }
        }
    }

    impl EndpointBuilder {
        pub fn new() -> Self {
            EndpointBuilder {
                name: None,
                path: None,
                query_params: vec![],
            }
        }

        pub fn with_name(mut self, name: &str) -> Self {
            self.name = Some(name.to_string());
            self
        }

        pub fn with_path(mut self, path: &str) -> Self {
            self.path = Some(path.to_string());
            self
        }

        pub fn with_query_param(mut self, name: &str) -> Self {
            self.query_params.push(name.to_string());
            self
        }

        pub fn build(self) -> Endpoint {
            Endpoint {
                name: self.name.unwrap(),
                path_template: self.path.unwrap(),
                query_parameters: self.query_params,
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
                headers: HashMap::new(),
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

        pub fn with_header(mut self, name: &str, value: &str) -> Self {
            self.headers.insert(name.to_string(), value.to_string());
            self
        }

        pub fn build(self) -> Environment {
            Environment {
                name: self.name.unwrap(),
                alias: self.alias,
                host: self.host.unwrap(),
                default: self.default,
                headers: self.headers,
            }
        }
    }
}