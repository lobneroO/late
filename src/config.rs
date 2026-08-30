
use iced::Theme;
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

pub fn save_config(config: &LateConfig) {
    let serialized = serde_json::to_string(&config);
    let config_file = match get_config_path(){
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
    let config_path = get_config_path().unwrap_or_default();
    let file_contents = fs::read_to_string(config_path);
    match file_contents {
        Ok(s) => serde_json::from_str(&s).unwrap_or_default(),
        Err(_) => LateConfig { theme: Theme::Dark, }
    }
}

