use crate::config::current_config::{Endpoint, Environment, HtrsConfig, Preset, QueryParameter, Service};
use crate::config::util::get_config_path;
use crate::config::versioned_config::versioned_htrs_config::VersionedHtrsConfig::{V1, V2};
use crate::config::versioned_config::versions::v1::v1config::HtrsConfigV1;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum VersionedHtrsConfig {
    V1(HtrsConfigV1),
    V2(HtrsConfig),
}

impl VersionedHtrsConfig {
    pub fn load_and_migrate_config() -> Result<HtrsConfig, String> {
        let path = get_config_path()?;
        let versioned_config = Self::load_config(&path)?;
        Ok(versioned_config.migrate_config())
    }

    fn load_config(path: &PathBuf) -> Result<VersionedHtrsConfig, String> {
        if !path.exists() {
            return Ok(V2(HtrsConfig::new()));
        }

        let file = match OpenOptions::new().read(true).open(path) {
            Ok(f) => f,
            Err(e) => return Err(format!("Failed to read config file A: {e}")),
        };

        match serde_json::from_reader(file) {
            Ok(config) => Ok(config),
            Err(e) => Err(format!("Failed to read config file B: {e}")),
        }
    }

    pub fn save_config(config: HtrsConfig) -> Result<(), String> {
        let path = get_config_path()?;
        let versioned_config = V2(config);

        let mut file = match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path) {
            Ok(f) => f,
            Err(e) => return Err(format!("Failed to write updated config file: {e}")),
        };

        match serde_json::to_writer_pretty(&mut file, &versioned_config) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to write updated config file: {e}")),
        }
    }

    fn migrate_config(&self) -> HtrsConfig {
        match self {
            V1(v1_config) => {
                let v2_config = HtrsConfig {
                    services: v1_config.services.iter()
                        .map(|v1_service| {
                            Service {
                                name: v1_service.name.clone(),
                                alias: v1_service.alias.clone(),
                                environments: v1_service.environments.iter()
                                    .map(|v1_env| {
                                        Environment {
                                            name: v1_env.name.clone(),
                                            alias: v1_env.alias.clone(),
                                            host: v1_env.host.clone(),
                                            headers: v1_env.headers.clone(),
                                            default: v1_env.default,
                                        }
                                    })
                                    .collect(),
                                endpoints: v1_service.endpoints.iter()
                                    .map(|v1_endpoint| {
                                        Endpoint {
                                            name: v1_endpoint.name.clone(),
                                            path_template: v1_endpoint.path_template.clone(),
                                            query_parameters: v1_endpoint.query_parameters.iter()
                                                .map(|v1_query_param| {
                                                    QueryParameter {
                                                        name: v1_query_param.name.clone(),
                                                        required: v1_query_param.required
                                                    }
                                                })
                                                .collect(),
                                        }
                                    })
                                    .collect(),
                                headers: v1_service.headers.clone(),
                            }
                        })
                        .collect(),
                    presets: v1_config.presets.iter()
                        .map(|v1_preset| {
                            Preset {
                                name: v1_preset.name.clone(),
                                alias: None,
                                values: v1_preset.values.clone(),
                            }
                        })
                        .collect(),
                    headers: v1_config.headers.clone(),
                };

                v2_config
            },
            V2(v2_config) => {
                v2_config.clone()
            }
        }
    }
}