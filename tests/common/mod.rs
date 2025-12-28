pub mod config;
pub mod builders;

#[cfg(test)]
pub mod test_helpers {
    use crate::common::builders::HtrsConfigBuilder;
    use crate::common::config::HtrsConfig;
    use std::fs::{remove_file, File, OpenOptions};
    use std::path::PathBuf;
    use uuid::Uuid;

    pub fn setup(init_config: Option<HtrsConfig>) -> String {
        let path_str = format!("{}.json", Uuid::new_v4());
        let path = PathBuf::from(path_str.clone());
        if path.exists() {
            remove_file(path.clone()).expect("Failed to clear existing config file");
        }

        let init_config = init_config.unwrap_or_else(|| HtrsConfigBuilder::new().build());
        let handle = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .unwrap();

        serde_json::to_writer_pretty(handle, &init_config).unwrap();

        path_str
    }

    pub fn get_config(path: &str) -> HtrsConfig {
        let path = PathBuf::from(path);
        serde_json::from_reader(File::open(path).unwrap()).unwrap()
    }

    pub fn clear_config(path: &str) {
        remove_file(path).expect("Failed to clean up test config file");
    }
}