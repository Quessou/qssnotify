use std::path;

use home::home_dir;

use super::constants::*;

pub fn get_app_directory_path() -> path::PathBuf {
    home_dir().unwrap().join(DEFAULT_APP_NAME)
}

pub fn get_data_file_path() -> path::PathBuf {
    get_app_directory_path().join(DATA_FILE_NAME)
}
pub fn get_config_file_path() -> path::PathBuf {
    get_app_directory_path().join(CONFIG_FILE_NAME)
}
