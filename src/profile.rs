
use std::path::PathBuf;
use std::fs::{self, File};
use std::io::Write;
use serde::{Serialize, Deserialize};

// TODO: save late's state (e.g. theme chosen)
// static CONFIG_NAME: &str = "late_config.json";
static PROFILES_NAME: &str = "late_profiles.json";

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
        config.push(PROFILES_NAME);

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

pub fn save_profiles(state: &Vec<LateProfile>) {
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

pub fn load_profiles() -> Vec<LateProfile> {
    let config_path = ensure_config_file().unwrap_or_default();
    print!("Trying to read {}", config_path.to_str().unwrap());
    let file_contents = fs::read_to_string(config_path)
        .expect("Could not read profiles file!");
    serde_json::from_str(&file_contents).unwrap_or(vec![])
}

pub fn get_profile_names(profiles: &Vec<LateProfile>) -> Vec<String> {
    let mut names = Vec::<String>::with_capacity(profiles.len());

    for profile in profiles {
        names.push(profile.name.clone());
    }
    names
}

pub fn choose_profile(profiles: &Vec<LateProfile>, name: &str) -> Option<LateProfile> {
    for profile in profiles {
        if profile.name == name {
            let copy = LateProfile {
                name: profile.name.clone(),
                sample_rate: profile.sample_rate,
                buffer_size: profile.buffer_size
            };
            return Some(copy);
        }
    }
    None
}

pub fn remove_profile(profiles: &mut Vec<LateProfile>, name: &str) {
    let index = profiles.iter().position(|x| x.name == name);
    if let Some(i) = index {
         profiles.remove(i);
    }
}

pub fn get_current_if_any(profiles: &Vec<LateProfile>, sample_rate: Option<u32>, buffer_size: Option<u32>) -> Option<String> {
    let s = sample_rate.unwrap_or(0);
    let b = buffer_size.unwrap_or(0);
    for profile in profiles {
        if profile.sample_rate == s
            && profile.buffer_size == b {
            // if there are multiple profiles with the same name, we return the first one.
            // there is no way to know which one the user wanted.
            return Some(profile.name.clone());
        }
    }
    None
}
