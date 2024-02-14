mod config;
mod init;
mod utils;

use std::path::Path;

use crate::init::init_repo;
use clap::{Arg, ArgAction, Command};

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
                        Some(Path::new(args.get_one::<String>("gitdir").unwrap()).to_path_buf())
                    } else {
                        None
                    },
                    if args.get_one::<String>("branch").is_some() {
                        Some(args.get_one::<String>("branch").unwrap())
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
