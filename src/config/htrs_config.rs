use crate::config::HtrsConfig;
use std::fs::OpenOptions;
use std::path::PathBuf;

impl HtrsConfig {
    pub fn load() -> HtrsConfig {
        let config_path = get_config_path();

        let mut options = &mut OpenOptions::new();
        options = match config_path.exists() {
            true => options.read(true),
            false => options.read(true).create(true),
        };
        let handle = options.open(config_path)
            .expect("Failed to open config file");

        serde_json::from_reader(handle)
            .expect("Failed to parse config.json")
    }

    pub fn save(&self) {
        let config_path = get_config_path();
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(config_path)
            .expect("Failed to write updated config.json");
        serde_json::to_writer_pretty(&mut file, &self)
            .expect("Failed to write updated config.json")
    }
}

fn get_config_path() -> PathBuf {
    std::env::current_exe()
        .expect("Unable to get executable location")
        .parent()
        .expect("Unable to get parent directory of executable")
        .join("config.json")
}
