use std::{fs::File, io::Read};

use colored::Colorize;
use flate2::read::ZlibDecoder;

use crate::utils::get_git_dir_path;

pub fn get_file_contents(blob_hash: String) -> String {
    let blob_path = get_git_dir_path()
        .join("objects")
        .join(&blob_hash[..2])
        .join(&blob_hash[2..]);

    let blob = File::open(blob_path).unwrap();

    let mut blob_decoder = ZlibDecoder::new(blob);
    let mut meta_and_contents = String::new();
    let _ = blob_decoder.read_to_string(&mut meta_and_contents);

    let (_, contents) = meta_and_contents
        .split_once("\x00")
        .ok_or(format!(
            "{}",
            "pgit: invalid hash. are you sure the hash points to a blob?".red()
        ))
        .unwrap();

    return contents.to_string();
}
