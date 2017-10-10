use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::collections::HashMap;

pub struct Config {
    pub input: String,
}

const LETTER_HEIGHT: usize = 17;
const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz.! ";
const OOZZ_HEIGHT: usize = 22;
const SYMBOLS: &str = "[]";
// const INIT: &str = "[0;1;40;32m";

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        // let input = args.clone()[1..].join(" ");
        let input = args[1..].join(" ");

        Ok(Config { input })
    }
}

fn parse_string<'a>(input: &'a str, letters: &str) -> HashMap<char, Vec<&'a str>> {

    let mut map = HashMap::new();

    for character in letters.chars().enumerate() {
        let (i, c) = character;
        let first = i * LETTER_HEIGHT;
        let last = first + LETTER_HEIGHT;
        let mut result = Vec::new();
        for n in first..last {
            let line = input.lines().nth(n).expect("Failed to retrieve line");
            result.push(line)
        }
        map.insert(c, result);
    }
    return map;
}

fn read_file(f: &str) -> Result<String, Box<Error>> {

    let mut file = File::open(format!("resources/{}.latin1", f))?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;

    Ok(string)
}

fn parse_oozz(input: &str) -> Vec<Vec<&str>> {
    let mut out: Vec<Vec<&str>> = Vec::new();
    let mut lines: Vec<&str> = input.lines().collect();
    while !lines.is_empty() {
        out.push(lines.drain(..OOZZ_HEIGHT).collect());
    }
    out
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let font = read_file("chars")?;
    let extra = read_file("extra")?;
    let oozz = read_file("oozz")?;
    let parsed_font = parse_string(&font[..], LETTERS);
    let parsed_extra = parse_string(&extra[..], SYMBOLS);
    let parsed_oozz = parse_oozz(&oozz[..]);
    let start = parsed_extra.get(&'[').ok_or("Couldn't retrive start character from parsed_extra")?;
    let stop = parsed_extra.get(&']').ok_or("Couldn't retrive end character from parsed_extra")?;

    let input = &config.input;
    let mut output = Vec::new();

    for n in 0..LETTER_HEIGHT {
        let mut line = String::from(start[n]);
        for input_char in input.chars() {
            let output_char = parsed_font.get(&input_char).ok_or("Couldn't retrieve character from parsed font")?;
            line = line + output_char[n];
        }
        line = line + stop[n];
        output.push(line)
    }

    for out in output {
        println!("{}", out);
    }

    println!("{:#?}", parsed_oozz);

//     fn titlecase_word(word: &str) -> String {
//         word.chars().enumerate()
//             .map(|(i, c)| if i == 0 { c.to_uppercase() } else { c.to_lowercase() })
//             .collect()
// }

    Ok(())
}


// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn case_sensitive() {
//         let query = "duct";
//         let contents = "\
//         Rust:
// safe, fast, productive.
// Pick three.
// Duct tape.";

//         assert_eq!(
//             vec!["safe, fast, productive."],
//             search(query, contents)
//         );
//     }

//     #[test]
//     fn case_insensitive() {
//         let query = "rUsT";
//         let contents = "\
//         Rust:
// safe, fast, productive.
// Pick three.
// Trust me.";

//         assert_eq!(
//             vec!["Rust:", "Trust me."],
//             search_case_insensitive(query, contents)
//         );
//     }
// }
