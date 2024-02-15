mod cat_file;
mod config;
mod init;
mod utils;

use cat_file::get_file_contents;
use clap::{builder::EnumValueParser, Arg, ArgAction, Command};
use colored::Colorize;
use init::init_repo;
use std::{env, path::Path};
use utils::HashAlgo;

fn main() {
    let init = Command::new("init")
        .about("Initialize or reinitialize a local git repository")
        .args([
            Arg::new("quiet")
                .long("quiet")
                .short('q')
                .action(ArgAction::SetTrue)
                .help("Don't output anything"),
            Arg::new("bare")
                .long("bare")
                .action(ArgAction::SetTrue)
                .help("Create files in current directory"),
            Arg::new("branch")
                .long("initial-branch")
                .short('b')
                .action(ArgAction::Set)
                .help("Override initial branch name")
                .value_name("name"),
            Arg::new("gitdir")
                .long("separate-git-dir")
                .action(ArgAction::Set)
                .help("Create repository in separate directory from working tree"),
            Arg::new("hashalgo")
                .long("object-format")
                .action(ArgAction::Set)
                .value_parser(EnumValueParser::<HashAlgo>::new())
                .value_name("hash"),
        ]);
    let cat_file = Command::new("cat-file")
        .about("Get file contents from object hash")
        .args([
            Arg::new("hash").required(true),
            Arg::new("pretty")
                .short('p')
                .help("Pretty print the contents of the blob")
                .action(ArgAction::SetTrue),
            Arg::new("exit")
                .short('e')
                .help("Check file validity")
                .action(ArgAction::SetTrue)
                .conflicts_with("pretty"),
        ]);

    let cli = Command::new("pgit")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("An alternative Git CLI which is actually understandable.")
        .subcommand(init)
        .subcommand(cat_file)
        .arg_required_else_help(true);

    match cli.try_get_matches() {
        Ok(matches) => match matches.subcommand_name() {
            Some("init") => {
                let args = matches.subcommand().unwrap().1.to_owned();
                init_repo(
                    Some(args.get_one::<bool>("quiet").unwrap().to_owned()),
                    Some(args.get_one::<bool>("bare").unwrap().to_owned()),
                    if args.get_one::<String>("gitdir").is_some() {
                        if !args.get_one::<String>("gitdir").unwrap().starts_with('/')
                            && !args
                                .get_one::<String>("gitdir")
                                .unwrap()
                                .split_at(1)
                                .1
                                .starts_with(':')
                        {
                            Some(env::current_dir().unwrap().join(
                                Path::new(args.get_one::<String>("gitdir").unwrap()).to_path_buf(),
                            ))
                        } else {
                            Some(Path::new(args.get_one::<String>("gitdir").unwrap()).to_path_buf())
                        }
                    } else {
                        None
                    },
                    if args.get_one::<String>("branch").is_some() {
                        Some(args.get_one::<String>("branch").unwrap())
                    } else {
                        None
                    },
                    if args.get_one::<HashAlgo>("hashalgo").is_some() {
                        Some(args.get_one::<HashAlgo>("hashalgo").unwrap().to_owned())
                    } else {
                        None
                    },
                );
            }
            Some("cat-file") => {
                let args = matches.subcommand().unwrap().1.to_owned();

                let output = get_file_contents(args.get_one::<String>("hash").unwrap().to_owned());

                if args.get_one::<bool>("pretty").unwrap().to_owned() {
                    println!("{}", output);
                }

                if args.get_one::<bool>("exit").unwrap().to_owned() {
                    if output
                        == String::from(format!(
                            "{}",
                            "pgit: invalid hash. are you sure the hash points to a blob?".red()
                        ))
                    {
                        println!("{}", output);
                    }
                }
            }
            _ => unreachable!("All exception cases are handled by clap"),
        },
        Err(err) => {
            if err.kind() == clap::error::ErrorKind::InvalidSubcommand {
                let iter = err.context().next();

                match iter {
                    Some((_, cmd)) => {
                        println!("pgit: '{cmd}' is not a valid command. See 'pgit --help'.")
                    }
                    None => {
                        println!("pgit: invalid command provided. See 'pgit --help'.")
                    }
                }
            } else {
                println!("{}", err.render())
            }
        }
    }
}
