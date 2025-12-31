
use std::path::PathBuf;

use directories::ProjectDirs;

static CONFIG_NAME: &str = "late_config.json";
static PROFILES_NAME: &str = "late_profiles.json";

fn get_base_path() -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("de", "new world labs", "late") {
        return Some(proj_dirs.config_dir().to_path_buf());
    }
    None
}

pub fn get_config_path() -> Option<PathBuf> {
    let base_path = get_base_path();
    match base_path {
        None => None,
        Some(mut p) => {
            p.push(CONFIG_NAME);
            Some(p)
        }
    }
}

pub fn get_profiles_path() -> Option<PathBuf> {

    let base_path = get_base_path();
    match base_path {
        None => None,
        Some(mut p) => {
            p.push(PROFILES_NAME);
            Some(p)
        }
    }
}
