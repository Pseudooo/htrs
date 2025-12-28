use std::env::{current_exe, var};
use std::path::PathBuf;

/// Get the config path for the application
///
/// Will first attempt to read from the `HTRS_CONFIG_PATH` environment variable, if set then that
/// path will be returned.
///
/// Otherwise, will look for config.json within the exe's directory.
pub fn get_config_path() -> Result<PathBuf, String> {
    if let Ok(path) = var("HTRS_CONFIG_PATH") {
        return Ok(PathBuf::from(path));
    }

    let exe_path = match current_exe() {
        Ok(path) => path,
        Err(e) => return Err(e.to_string()),
    };

    let directory = match exe_path.parent() {
        Some(path) => path,
        None => return Err(format!("No parent directory could be found for path `{}`", exe_path.display())),
    };

    Ok(directory.join("config.json"))
}
