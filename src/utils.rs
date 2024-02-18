use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
    process::exit,
};

use clap::ValueEnum;

pub fn get_git_dir_path() -> PathBuf {
    let cwd = env::current_dir();
    const GIT_DIR: &str = ".git";

    match cwd {
        Ok(path) => {
            return Path::new(path.as_os_str()).join(GIT_DIR);
        }
        Err(_) => {
            eprintln!("error occured in init::check_git_dir()");
            exit(1);
        }
    }
}

pub fn merge_hashmaps(
    hm_1: HashMap<String, HashMap<String, Option<String>>>,
    hm_2: HashMap<String, HashMap<String, Option<String>>>,
) -> HashMap<String, HashMap<String, Option<String>>> {
    let mut final_hash: HashMap<String, HashMap<String, Option<String>>> = HashMap::new();

    for (key, val) in hm_1 {
        if final_hash.contains_key(&key) {
            let mut inner_final_hash: HashMap<String, Option<String>> = HashMap::new();

            for (key_i, val_i) in val {
                inner_final_hash.insert(key_i, val_i);
            }

            for (key_i, val_i) in &final_hash[&key] {
                inner_final_hash.insert(key_i.to_string(), val_i.clone());
            }

            final_hash.insert(key, inner_final_hash);
        } else {
            final_hash.insert(key, val);
        }
    }

    for (key, val) in hm_2 {
        if final_hash.contains_key(&key) {
            let mut inner_final_hash: HashMap<String, Option<String>> = HashMap::new();

            for (key_i, val_i) in val {
                inner_final_hash.insert(key_i, val_i);
            }

            for (key_i, val_i) in &final_hash[&key] {
                inner_final_hash.insert(key_i.to_string(), val_i.clone());
            }

            final_hash.insert(key, inner_final_hash);
        } else {
            final_hash.insert(key, val);
        }
    }

    return final_hash;
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum HashAlgo {
    Sha1,
    Sha256,
}

unsafe impl Send for HashAlgo {}
unsafe impl Sync for HashAlgo {}
