
use iced::Theme;
use std::path::PathBuf;
use std::fs::{self, File};
use std::io::Write;
use serde::{Serialize, Deserialize};
use crate::paths::get_config_path;
use crate::serde_helper::ThemeDef;

#[derive(Default, Serialize, Deserialize)]
pub struct LateConfig {
    #[serde(with = "ThemeDef")]
    pub theme: Theme,
}

// TODO: pretty much the same function as ensure_profiles_file, 
// combine the shared code
pub fn ensure_config_file() -> std::io::Result<PathBuf> {
    let config_path_opt = get_config_path();
    
    if let Some(config_path) = config_path_opt {

        // ensure we can fetch the config dir and exists state
        let config_dir_exists = fs::exists(&config_path);
        if config_dir_exists.is_err() {
            return Err(config_dir_exists.err().unwrap());
        }

        // ensure we have a config directory
        if !fs::exists(&config_path).unwrap() {
            let result = fs::create_dir(&config_path);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
        }
    }

    // ensure we can fetch the config file and its exists state
    let config_path_opt = get_config_path();
    if let Some(config_path) = config_path_opt {

        let config_file_exists = fs::exists(&config_path);
        if config_file_exists.is_err() {
            return Err(config_file_exists.err().unwrap());
        }

        // ensure we have a config file
        if !fs::exists(&config_path).unwrap() {
            let result = File::create(&config_path);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
        }

        return Ok(config_path);
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Cannot find home directory!"))
}

pub fn save_config(config: &LateConfig) {
    let serialized = serde_json::to_string(&config);
    let config_file = match ensure_config_file(){
        Ok(c) => c,
        Err(e) => { 
            print!("{}", e);
            return;
        }
    };

    let f = File::create(config_file);
    write!(f.unwrap(), "{}", serialized.unwrap())
        .expect("Could not write config to file!");

}

pub fn load_config() -> LateConfig {
    let config_path = ensure_config_file().unwrap_or_default();
    let file_contents = fs::read_to_string(config_path);
    match file_contents {
        Ok(s) => serde_json::from_str(&s).unwrap_or_default(),
        Err(_) => LateConfig { theme: Theme::Dark, }
    }
}

