extern crate oozz;
extern crate clap;

use std::process;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Oozz")
        .version("0.1.0")
        .author("Daniel Berg <mail@roosta.sh>")
        .about("ANSI art font rendering")
        .arg(Arg::with_name("color")
             .short("c")
             .long("color")
             .value_name("color")
             .takes_value(true)
             .help("color for oozz, can be one of seven predefined: black, red, green, yellow, blue, magenta, cyan, white."))
        .arg(Arg::with_name("INPUT")
             .help(&format!("Input string to render, accepted chars are currently: {}",
                            oozz::LETTERS)[..])
             .required(true)
             .multiple(true))
        .get_matches();

    if let Err(e) = oozz::run(matches) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
