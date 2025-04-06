
use iced::Theme;
use std::path::PathBuf;
use std::fs::{self, File};
use std::io::Write;
use serde::{Serialize, Deserialize};
use crate::serde_helper::ThemeDef;

use crate::paths::CONFIG_PATH;
use crate::paths::CONFIG_NAME;

#[derive(Default, Serialize, Deserialize)]
pub struct LateConfig {
    #[serde(with = "ThemeDef")]
    pub theme: Theme,
}

// TODO: pretty much the same function as ensure_profiles_file, 
// combine the shared code
pub fn ensure_config_file() -> std::io::Result<PathBuf> {
    let home_opt = home::home_dir();
    if home_opt.is_some() {
        let mut config = home_opt.unwrap();
        config.push(CONFIG_PATH);

        // ensure we can fetch the config dir and exists state
        let config_dir_exists = fs::exists(&config);
        if config_dir_exists.is_err() {
            return Err(config_dir_exists.err().unwrap());
        }

        // ensure we have a config dir 
        if !fs::exists(&config).unwrap() {
            let result = fs::create_dir(&config);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
        }

        // ensure we can fetch the config file and its exists state
        config.push(CONFIG_NAME);

        let config_file_exists = fs::exists(&config);
        if config_file_exists.is_err() {
            return Err(config_file_exists.err().unwrap());
        }

        // ensure we have a config file
        if !fs::exists(&config).unwrap() {
            let result = File::create(&config);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
        }

        return Ok(config);
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

