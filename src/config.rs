use crate::utils::get_git_dir_path;
use home::home_dir;

pub enum ConfigLocation {
    Local,
    Global,
    System,
    WinUserRoot,
    Unknown,
}

pub fn locate_config_file() -> ConfigLocation {
    let base_git_dir = get_git_dir_path();
    let local_config_path = base_git_dir.join("config");

    if local_config_path.exists() {
        return ConfigLocation::Local;
    }

    match home_dir() {
        Some(path) => {
            if path.join(".gitconfig").exists()
                || path.join(".config").join("git").join("config").exists()
            {
                return ConfigLocation::Global;
            } else {
                return ConfigLocation::System;
            }
        }
        _ => {
            return ConfigLocation::Unknown;
        }
    }
}
