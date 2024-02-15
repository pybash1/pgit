use crate::config::get_config_value;
use crate::utils::get_git_dir_path;
use crate::utils::HashAlgo;
use std::path::PathBuf;
use std::{env, fs};

fn already_init() -> bool {
    let mut inited = false;

    if get_git_dir_path().exists() {
        inited = true;
    }

    return inited;
}

pub fn init_repo(
    quiet: Option<bool>,
    bare: Option<bool>,
    separate_git_dir: Option<PathBuf>,
    branch_name: Option<&str>,
    hashing_algo: Option<HashAlgo>
) {
    let base_git_dir = if separate_git_dir.is_some() {
        if !bare.unwrap() {
            separate_git_dir.unwrap().join(".git")
        } else {
            separate_git_dir.unwrap()
        }
    } else {
        if !bare.unwrap() {
            get_git_dir_path()
        } else {
            env::current_dir().unwrap()
        }
    };
    let objects_git_dir = base_git_dir.join("objects");
    let objects_info_git_dir = objects_git_dir.join("info");
    let objects_pack_git_dir = objects_git_dir.join("pack");
    let refs_git_dir = base_git_dir.join("refs");
    let refs_heads_git_dir = refs_git_dir.join("heads");
    let refs_tags_git_dir = refs_git_dir.join("tags");
    let hooks_git_dir = base_git_dir.join("hooks");
    let info_git_dir = base_git_dir.join("info");
    let head_git_file = base_git_dir.join("HEAD");
    let config_git_file = base_git_dir.join("config");
    let description_git_file = base_git_dir.join("description");

    if already_init() {
        println!(
            "Reinitialized existing Git repository in {}.",
            get_git_dir_path().into_os_string().to_string_lossy()
        );
    } else {
        fs::create_dir_all(base_git_dir).unwrap();
        fs::create_dir_all(objects_git_dir).unwrap();
        fs::create_dir_all(objects_info_git_dir).unwrap();
        fs::create_dir_all(objects_pack_git_dir).unwrap();
        fs::create_dir_all(refs_git_dir).unwrap();
        fs::create_dir_all(refs_heads_git_dir).unwrap();
        fs::create_dir_all(refs_tags_git_dir).unwrap();
        fs::create_dir_all(hooks_git_dir).unwrap();
        fs::create_dir_all(info_git_dir).unwrap();

        let def_branch = get_config_value(String::from("init.defaultbranch"));

        fs::write(
            head_git_file,
            String::from("ref: refs/heads/")
                + branch_name.unwrap_or(def_branch.unwrap_or(String::from("master")).as_str())
                + "\n",
        )
        .unwrap();

        let mut config = String::from("");

        if hashing_algo.is_some() && hashing_algo.unwrap() == HashAlgo::Sha256 {
            config += "[extensions]\n\tobjectformat = sha256";
        }

        fs::write(config_git_file, config).unwrap();
        fs::write(
            description_git_file,
            "Unnamed repository; edit this file 'description' to name the repository.",
        )
        .unwrap();

        if !quiet.unwrap() {
            println!(
                "Initialized empty Git repository in {}.",
                get_git_dir_path().into_os_string().to_string_lossy()
            );
        }
    }
}
