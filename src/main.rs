// (Full example with detailed comments in examples/17_yaml.rs)
#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

}
