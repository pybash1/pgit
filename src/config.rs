use crate::utils::{get_git_dir_path, merge_hashmaps};
use home::home_dir;
use ini::configparser::ini::Ini;
use std::{collections::HashMap, env, path::Path};

#[derive(Debug)]
pub enum ConfigLocation {
    Local,
    Global,
    System,
}

pub fn get_merged_config() -> (
    HashMap<String, HashMap<String, Option<String>>>,
    HashMap<String, ConfigLocation>,
) {
    let base_git_dir = get_git_dir_path();
    let local_config_path = base_git_dir.join("config");
    let global_config_path = home_dir()
        .unwrap()
        .join(".config")
        .join("git")
        .join("config");
    let global_config_path_2 = home_dir().unwrap().join(".gitconfig");

    let system_config_path = if cfg!(windows) {
        Path::new(env::var("ProgramFiles").unwrap().as_str())
            .join("Git")
            .join("etc")
            .join("gitconfig")
    } else {
        Path::new("/etc").join("gitconfig")
    };

    let mut config: HashMap<String, HashMap<String, Option<String>>> = HashMap::new();
    let mut locs: HashMap<String, ConfigLocation> = HashMap::new();

    if system_config_path.exists() {
        let mut system_config = Ini::new();
        match system_config.load(system_config_path.to_str().unwrap()) {
            Ok(sconfig) => {
                for (k, v) in &sconfig {
                    for (k_i, _) in v {
                        locs.insert(k.to_string() + "." + k_i.as_str(), ConfigLocation::System);
                    }
                }
                config = merge_hashmaps(config, sconfig);
            }
            _ => {}
        }
    }

    if global_config_path.exists() {
        let mut global_config = Ini::new();
        match global_config.load(global_config_path.to_str().unwrap()) {
            Ok(gconfig) => {
                for (k, v) in &gconfig {
                    for (k_i, _) in v {
                        locs.insert(k.to_string() + "." + k_i.as_str(), ConfigLocation::Global);
                    }
                }
                config = merge_hashmaps(config, gconfig);
            }
            _ => {}
        }
    } else if global_config_path_2.exists() {
        let mut global_config = Ini::new();
        match global_config.load(global_config_path_2.to_str().unwrap()) {
            Ok(gconfig) => {
                for (k, v) in &gconfig {
                    for (k_i, _) in v {
                        locs.insert(k.to_string() + "." + k_i.as_str(), ConfigLocation::Global);
                    }
                }
                config = merge_hashmaps(config, gconfig);
            }
            _ => {}
        }
    }

    if local_config_path.exists() {
        let mut local_config = Ini::new();
        match local_config.load(local_config_path.to_str().unwrap()) {
            Ok(lconfig) => {
                for (k, v) in &lconfig {
                    for (k_i, _) in v {
                        locs.insert(k.to_string() + "." + k_i.as_str(), ConfigLocation::Local);
                    }
                }
                config = merge_hashmaps(config, lconfig);
            }
            _ => {}
        }
    }

    // for debugging purposes
    // for (k, v) in &config {
    //     for (k_i, v_i) in v {
    //         println!(
    //             "{:?} {}.{}={}",
    //             locs[&k.clone().add(".").add(k_i)],
    //             k,
    //             k_i,
    //             v_i.clone().unwrap()
    //         );
    //     }
    // }

    return (config, locs);
}

pub fn get_config_value(mut key: String) -> String {
    key += ".";
    let config = get_merged_config().0;

    let config_vec: Vec<&str> = key.split(".").collect();

    let section = config
        .get(config_vec[0])
        .unwrap_or(&HashMap::new())
        .to_owned();
    let value = section
        .get(config_vec[1])
        .unwrap_or(&Some(String::from("")))
        .to_owned()
        .unwrap_or(String::from(""));

    return value;
}
