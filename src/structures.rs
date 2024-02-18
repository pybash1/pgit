use colored::Colorize;
use flate2::read::ZlibDecoder;
use hex;
use std::{
    fmt::{Display, Formatter, Result},
    fs::File,
    io::Read,
    path::PathBuf,
    process::exit,
};

use crate::utils::get_git_dir_path;

fn handler<T>(msg: &str) -> T {
    println!("{}", msg.red());
    exit(1);
}

#[derive(Debug, PartialEq)]
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
    Tag,
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ObjectType::Blob => {
                write!(f, "blob")
            }
            ObjectType::Tree => {
                write!(f, "tree")
            }
            ObjectType::Commit => {
                write!(f, "commit")
            }
            ObjectType::Tag => {
                write!(f, "tag")
            }
        }
    }
}

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub email: Option<String>,
    pub time: u64,
    pub zone: String,
}

#[derive(Debug)]
pub struct Meta {
    pub objects: Option<Vec<Object>>,
    pub tree: Option<String>,
    pub parent: Option<String>,
    pub author: Option<User>,
    pub committer: Option<User>,
    pub mode: Option<String>,
    pub filename: Option<String>,
}

#[derive(Debug)]
pub struct Object {
    pub obj_type: ObjectType,
    pub hash: String,
    pub size: u32,
    pub meta: Option<Meta>,
}

impl Object {
    pub fn new(hash: String) -> Self {
        let object_path = get_git_dir_path()
            .join("objects")
            .join(&hash[..2])
            .join(&hash[2..]);

        let object =
            File::open(object_path).unwrap_or_else(|_| handler::<File>("failed to read file"));

        let mut decoder = ZlibDecoder::new(object);
        let mut data = Vec::<u8>::new();

        let _ = decoder.read_to_end(&mut data);

        let mut data = data.split(|n| n == &0).to_owned();

        let header = String::from_utf8_lossy(
            data.next()
                .unwrap_or_else(|| handler::<&[u8]>("failed to get header bytes")),
        );

        let mut header = header.trim().split_whitespace();

        let header_type = &header
            .next()
            .unwrap_or_else(|| handler::<&'static str>("invalid header found"))
            .to_owned()
            .as_str()
            .to_owned();

        let type_ = if header_type == "blob" {
            ObjectType::Blob
        } else if header_type == "tree" {
            ObjectType::Tree
        } else if header_type == "tag" {
            ObjectType::Tag
        } else {
            ObjectType::Commit
        };

        let tree_meta = if type_ == ObjectType::Tree {
            let mut tree_body = String::from_utf8_lossy(
                data.next()
                    .unwrap_or_else(|| handler::<&[u8]>("failed to get body")),
            )
            .to_string();

            loop {
                let r = data.next();
                if r == None {
                    break;
                } else {
                    tree_body += "\x00";
                    let r = r.unwrap_or_else(|| handler::<&[u8]>("failed to get header bytes"));
                    tree_body += &hex::encode(&r[0..20]);
                    tree_body += &String::from_utf8_lossy(&r[20..]).to_string();
                }
            }

            let mut tree_objs = Vec::<Object>::new();

            let mut mode_filename = String::new();
            let mut next_mode_filename = String::new();
            let mut obj_hash = String::new();

            for part in tree_body.split("\x00") {
                if part.len() < 20 {
                    mode_filename = part.to_string();
                } else if part.len() == 20 {
                    obj_hash = part.to_string();
                } else {
                    obj_hash = part[0..40].to_string();
                    if !next_mode_filename.is_empty() {
                        mode_filename = next_mode_filename;
                    }
                    next_mode_filename = part[40..].to_string();
                }

                if !mode_filename.is_empty() && !obj_hash.is_empty() {
                    tree_objs.push(Object {
                        obj_type: ObjectType::Blob,
                        hash: obj_hash.clone(),
                        size: 69,
                        meta: Some(Meta {
                            objects: None,
                            tree: None,
                            parent: None,
                            author: None,
                            committer: None,
                            mode: Some(mode_filename.split_once(" ").unwrap().0.to_string()),
                            filename: Some(mode_filename.split_once(" ").unwrap().1.to_string()),
                        }),
                    });
                }
            }

            Some(tree_objs)
        } else {
            None
        };

        let commit_meta = if type_ == ObjectType::Commit {
            let dets = String::from_utf8_lossy(
                data.next()
                    .unwrap_or_else(|| handler::<&[u8]>("failed to get body")),
            )
            .to_string();
            let mut dets = dets.split("\n");

            let tree = dets
                .next()
                .unwrap_or_else(|| handler::<&str>("failed to get commit info"))
                .split_once(" ")
                .unwrap_or_else(|| handler::<(&str, &str)>("failed to get commit info"))
                .1;
            let parent = dets
                .next()
                .unwrap_or_else(|| handler::<&str>("failed to get commit info"))
                .split_once(" ")
                .unwrap_or_else(|| handler::<(&str, &str)>("failed to get commit info"))
                .1;
            let author = dets
                .next()
                .unwrap_or_else(|| handler::<&str>("failed to get commit info"))
                .split_once(" ")
                .unwrap_or_else(|| handler::<(&str, &str)>("failed to get commit info"))
                .1;
            let committer = dets
                .next()
                .unwrap_or_else(|| handler::<&str>("failed to get commit info"))
                .split_once(" ")
                .unwrap_or_else(|| handler::<(&str, &str)>("failed to get commit info"))
                .1;

            let mut author = author.split_whitespace();
            let mut committer = committer.split_whitespace();

            let author_name = author
                .next()
                .unwrap_or_else(|| handler::<&str>("failed to get commit info"));
            let author_email = author
                .next()
                .unwrap_or_else(|| handler::<&str>("failed to get commit info"));
            let author_time = author
                .next()
                .unwrap_or_else(|| handler::<&str>("failed to get commit info"));
            let author_zone = author
                .next()
                .unwrap_or_else(|| handler::<&str>("failed to get commit info"));
            let committer_name = committer
                .next()
                .unwrap_or_else(|| handler::<&str>("failed to get commit info"));
            let committer_email = committer
                .next()
                .unwrap_or_else(|| handler::<&str>("failed to get commit info"));
            let committer_time = committer
                .next()
                .unwrap_or_else(|| handler::<&str>("failed to get commit info"));
            let committer_zone = committer
                .next()
                .unwrap_or_else(|| handler::<&str>("failed to get commit info"));

            let author_email = &author_email[1..author_email.len() - 1];
            let committer_email = &committer_email[1..author_email.len() - 1];

            let author = User {
                name: author_name.to_owned(),
                email: Some(author_email.to_owned()),
                time: author_time.parse().unwrap(),
                zone: author_zone.to_owned(),
            };
            let committer = User {
                name: committer_name.to_owned(),
                email: Some(committer_email.to_owned()),
                time: committer_time.parse().unwrap(),
                zone: committer_zone.to_owned(),
            };

            (
                Some(tree.to_owned()),
                Some(parent.to_owned()),
                Some(author),
                Some(committer),
            )
        } else {
            (None, None, None, None)
        };

        Self {
            obj_type: type_,
            hash: hash,
            size: header.next().unwrap().to_owned().parse().unwrap(),
            meta: Some(Meta {
                objects: tree_meta,
                tree: commit_meta.0,
                parent: commit_meta.1,
                author: commit_meta.2,
                committer: commit_meta.3,
                mode: None,
                filename: None,
            }),
        }
    }

    fn get_path(&self) -> PathBuf {
        get_git_dir_path()
            .join("objects")
            .join(&self.hash[..2])
            .join(&self.hash[2..])
    }

    pub fn get_contents(&self) -> String {
        let path = self.get_path();

        let object = File::open(path).unwrap_or_else(|_| handler::<File>("failed to read file"));
        let mut decoder = ZlibDecoder::new(object);
        let mut contents = Vec::<u8>::new();

        let _ = decoder.read_to_end(&mut contents);

        let mut contents = contents.split(|n| n == &0);
        contents.next();

        if self.obj_type == ObjectType::Tree {
            let mut tree_body = String::from_utf8_lossy(
                contents
                    .next()
                    .unwrap_or_else(|| handler::<&[u8]>("failed to get body")),
            )
            .to_string();

            loop {
                let r = contents.next();
                if r == None {
                    break;
                } else {
                    tree_body += "\n";
                    let r = r.unwrap_or_else(|| handler::<&[u8]>("failed to get header bytes"));
                    tree_body += &hex::encode(&r[0..20]);
                    tree_body += &String::from_utf8_lossy(&r[20..]).to_string();
                }
            }

            return tree_body;
        }

        String::from_utf8_lossy(
            contents
                .next()
                .unwrap_or_else(|| handler::<&[u8]>("unreadable file contents")),
        )
        .trim()
        .to_owned()
    }
}
