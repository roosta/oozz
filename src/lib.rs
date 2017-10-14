extern crate rand;
extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::collections::HashMap;
use rand::Rng;

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

fn parse_oozz(input: &str) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = Vec::new();
    let mut padded: Vec<String> = Vec::new();
    let all_escape_re = Regex::new(r"\x1b[^m]*m").unwrap();
    let cursor_forward_re = Regex::new(r"(\x1b\[)([0-9]+)(C)").unwrap();
    for line in input.lines() {
        let trimmed = all_escape_re.replace_all(line, "");
        let mut captured_padding = 0;
        for cap in cursor_forward_re.captures_iter(line) {
            captured_padding = captured_padding + &cap[2].parse::<usize>().expect("Failed to parse cursor_forward_re capture");
        }
        let count = trimmed.chars().count();
        let pad_count = 18 - count - captured_padding;
        let pad: String = (0..pad_count).map(|_| " ").collect();
        padded.push(String::from(line) + &pad[..]);
    }
    while !padded.is_empty() {
        out.push(padded.drain(..OOZZ_HEIGHT).collect());
    }
    out
}

// fn produce_output() {

// }

fn choose_oozz(input: &str, oozz: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut rng = rand::thread_rng();
    let mut out = Vec::new();
    for _ in input.chars() {
        let chosen = rng.choose(&oozz).expect("Failed to randomly choose an oozz character from parsed");
        out.push(chosen.to_vec());
    }
    out
}

pub fn run(config: Config) -> Result<(), Box<Error>> {

    let chars = read_file("chars")?;
    let extra = read_file("extra")?;
    let oozz = read_file("oozz")?;

    let chars = parse_string(&chars[..], LETTERS);
    let extra = parse_string(&extra[..], SYMBOLS);
    let oozz = parse_oozz(&oozz[..]);

    let oozz_stop = "─┘";
    let oozz_start = "└─";

    let chars_start = extra.get(&'[').ok_or("Couldn't retrive start character from extra")?;
    let chars_stop = extra.get(&']').ok_or("Couldn't retrive end character from extra")?;

    let input = &config.input;

    let oozz = choose_oozz(&input, &oozz);

    // println!("{:#?}", oozz);

    let mut output = Vec::new();

    for n in 0..LETTER_HEIGHT {
        let mut line = String::from(chars_start[n]);
        for input_char in input.chars() {
            let output_char = chars.get(&input_char).ok_or("Failed to retrieve character from chars")?;
            line = line + output_char[n];
        }
        line = line + chars_stop[n];
        output.push(line)
    }
    for n in 0..OOZZ_HEIGHT {
        let mut line = String::new();
        if n == 0 {
           line = line + oozz_start;
        } else {
            line = line + "  ";
        }
        for input_char in input.chars().enumerate() {
            let (i, _) = input_char;
            let output_char = oozz.get(i).ok_or("Failed to retrieve character from oozz")?;
            line = line + &output_char[n][..];
        }
        if n == 0 {
            line = line + oozz_stop;
        } else {
            line = line + "  ";
        }
        output.push(line);
    }

    for out in output {
        println!("{}", out);
    }


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
