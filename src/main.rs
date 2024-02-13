mod config;
mod init;
mod utils;

use crate::init::init_repo;
use clap::Command;

fn main() {
    let cli = Command::new("pgit")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("An alternative Git CLI which is actually understandable.")
        .subcommand(Command::new("init").about("Initialize or reinitialize a local git repository"))
        .arg_required_else_help(true);

    match cli.try_get_matches() {
        Ok(matches) => match matches.subcommand_name() {
            Some("init") => {
                init_repo(None, None, None, None);
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
