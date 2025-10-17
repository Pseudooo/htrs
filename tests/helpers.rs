use serde_json::Value;
use std::fs::remove_file;
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

pub fn setup() {
    let path = get_config_path();
    if path.exists() {
        remove_file(path).expect("Failed to clear existing config file");
    }
}

pub fn get_config() -> Value {
    serde_json::from_reader(std::fs::File::open(get_config_path()).unwrap()).unwrap()
}