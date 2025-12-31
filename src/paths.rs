
use std::{fs, path::PathBuf};

use directories::ProjectDirs;

static CONFIG_NAME: &str = "late_config.json";
static PROFILES_NAME: &str = "late_profiles.json";


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
        let res = fs::create_dir_all(proj_dirs.config_dir().to_path_buf())?;
        res
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Cannot find home directory!"))
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
