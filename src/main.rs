extern crate oozz;
extern crate clap;
extern crate term_size;

use std::process;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Oozz")
        .version("0.3.3")
        .author("Daniel Berg <mail@roosta.sh>")
        .about("ANSI art font rendering, with some added oozz")
        .arg(Arg::with_name("bold")
             .short("b")
             .long("bold")
             .help("Use bold colors"))
        .arg(Arg::with_name("center")
             .short("C")
             .long("center")
             .help("Center output to terminal width"))
        .arg(Arg::with_name("color")
             .short("c")
             .long("color")
             .value_name("color")
             .takes_value(true)
             .possible_values(&["black", "red", "green", "yellow", "blue", "magenta", "cyan", "white"])
             .help("Oozz color"))
        .arg(Arg::with_name("INPUT")
             .help(&format!("Input string to render, accepted chars are currently: {}",
                            oozz::LETTERS)[..])
             .required(true)
             .validator(oozz::valid_chars)
             .multiple(true))
        .get_matches();

    let input: Vec<&str> = matches.values_of("INPUT").unwrap().collect();
    let input  = input.join(" ");
    let color  = matches.value_of("color").unwrap_or("green");
    let bold   = matches.is_present("bold");
    let mut center =  matches.is_present("center");
    let term_width = match term_size::dimensions() {
        Some((term_width, _)) => term_width,
        _ => {
            eprintln!("Failed to get terminal dimensions, countinuing with no centering.");
            center = false;
            0
        }
    };
    match oozz::run(&input, &color, bold, center, term_width) {
        Ok(result) => {
            for l in result {
                println!("{}", l);
            }
        }
        Err(e) => {
            eprintln!("Application error: {}", e);

            process::exit(1);
        }
    }
}
