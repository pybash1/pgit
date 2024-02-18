use clap::ArgMatches;

use crate::structures::Object;

pub fn debug(args: ArgMatches) {
    let arg = args.get_one::<String>("arg").unwrap().to_owned();
    println!("{:#?}", arg);

    let object = Object::new(arg);

    println!("{:#?}", object.get_contents());
}
