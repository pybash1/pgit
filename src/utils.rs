use std::{
    env,
    path::{Path, PathBuf},
    process::exit,
};

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
