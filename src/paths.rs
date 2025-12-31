
use std::{fs, path::PathBuf};

use directories::ProjectDirs;

use crate::config;

static CONFIG_NAME: &str = "config.json";
static PROFILES_NAME: &str = "profiles.json";


fn ensure_file(path: &PathBuf) -> std::io::Result<()> {
    if fs::exists(path)? {
        Ok(())
    }
    else {
        let res = fs::File::create(path);
        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }
}

fn get_base_path() -> std::io::Result<PathBuf> {
    // ensure we have the config directory - either it is accessible, or we create it.
    if let Some(proj_dirs) = ProjectDirs::from("de", "new world labs", "late") {
        // create_dir_all will create all folders in the path that are missing,
        // but won't delete any already existing dirs (so may create no folder at all)
        let config_dir = proj_dirs.config_dir();
        let res = fs::create_dir_all(config_dir);
        match res {
            Ok(_) => {
                return Ok(config_dir.to_path_buf());
            },
            Err(err) => {
                return Err(err)
            }
        }
    }
    Err(std::io::Error::other("Cannot find home directory!"))
}

pub fn get_config_path() -> std::io::Result<PathBuf> {
    let path_res = get_base_path();
    match path_res {
        Ok(mut path) => {
            path.push(CONFIG_NAME);
            let f = ensure_file(&path);
            match f {
                Ok(_) => {
                    Ok(path)
                }
                Err(err) => Err(err)
            }
        },
        Err(err) => Err(err)
    }
}

pub fn get_profiles_path() -> std::io::Result<PathBuf> {
    let path_res = get_base_path();
    match path_res {
        Ok(mut path) => {
            path.push(PROFILES_NAME);
            let f = ensure_file(&path);
            match f {
                Ok(_) => {
                    Ok(path)
                }
                Err(err) => Err(err)
            }
        },
        Err(err) => Err(err)
    }
}
