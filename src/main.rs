mod config;
mod init;
mod utils;

use clap::{builder::EnumValueParser, Arg, ArgAction, Command};
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

    let cli = Command::new("pgit")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("An alternative Git CLI which is actually understandable.")
        .subcommand(init)
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
