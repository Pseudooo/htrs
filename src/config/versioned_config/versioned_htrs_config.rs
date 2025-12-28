use crate::config::current_config::HtrsConfig;
use crate::config::util::get_config_path;
use crate::config::versioned_config::migrations::migrate_v1_to_v2::migrate_v1_to_v2;
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

    fn migrate_config(self) -> HtrsConfig {
        match self {
            V1(v1_config) => {
                migrate_v1_to_v2(v1_config)
            },
            V2(v2_config) => {
                v2_config
            }
        }
    }
}