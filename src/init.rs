use crate::utils::get_git_dir_path;
use std::fs;
use std::path::PathBuf;

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
) {
    let base_git_dir = get_git_dir_path();
    let objects_git_dir = base_git_dir.join("objects");
    let objects_info_git_dir = objects_git_dir.join("info");
    let objects_pack_git_dir = objects_git_dir.join("pack");
    let refs_git_dir = base_git_dir.join("refs");
    let refs_heads_git_dir = refs_git_dir.join("heads");
    let refs_tags_git_dir = refs_git_dir.join("tags");
    let hooks_git_dir = base_git_dir.join("hooks");
    let info_git_dir = base_git_dir.join("info");
    let head_git_file = base_git_dir.join("HEAD");
    let fetch_head_git_file = base_git_dir.join("FETCH_HEAD");
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
        fs::write(head_git_file, "ref: refs/heads/master\n").unwrap();
        fs::write(fetch_head_git_file, "").unwrap();
        fs::write(config_git_file, "").unwrap();
        fs::write(
            description_git_file,
            "Unnamed repository; edit this file 'description' to name the repository.",
        )
        .unwrap();

        if let Some(_) = quiet {
            println!(
                "Initialized empty Git repository in {}.",
                get_git_dir_path().into_os_string().to_string_lossy()
            );
        }
    }
}
