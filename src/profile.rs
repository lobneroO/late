
use std::path::PathBuf;
use std::fs::{self, File};
use std::io::Write;
use serde::{Serialize, Deserialize};

static CONFIG_NAME: &str = "late_config.json";

/// Extra state which copies LateState::buffer_size and LateState::sample_rate
/// in order to easily serialize and deserialize them.
/// Serialization is meant for profiles a user may create.
/// E.g: A recording profile (with low latency) and a mixing / everyday profile
/// (with moderate latency allowing for larger buffer sizes)
#[derive(Serialize, Deserialize)]
pub struct LateProfile {
    /// the name under which to store the profile
    pub name: String,
    /// the buffer size
    pub buffer_size: u32,
    /// the sample rate
    pub sample_rate: u32
}

pub fn ensure_config_file() -> std::io::Result<PathBuf> {
    let home_opt = home::home_dir();
    if home_opt.is_some() {
        let mut config = home_opt.unwrap();
        config.push(".config/late");

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

pub fn save_state(state: LateProfile) {
    let serialized = serde_json::to_string(&state);
    let config_file = match ensure_config_file(){
        Ok(c) => c,
        Err(e) => { 
            print!("{}", e);
            return;
        }
    };

    let f = File::create(config_file);
    write!(f.unwrap(), "{}", serialized.unwrap());

}

pub fn load_profiles() -> Vec<String> {
    let profiles = vec!["test1".to_string(), "test".to_string()];

    profiles
}


