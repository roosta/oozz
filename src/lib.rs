extern crate rand;
extern crate regex;
extern crate clap;

use regex::Regex;
use std::error::Error;
use std::collections::HashMap;
use rand::Rng;

// Define letter and oozz height
const LETTER_HEIGHT: usize = 17;
const OOZZ_HEIGHT: usize = 22;

// Available letters in font
pub const LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyz.! ";

const CHARS: &'static str = include_str!("../resources/chars.latin1");
const EXTRA: &'static str = include_str!("../resources/extra.latin1");
const OOZZ: &'static str = include_str!("../resources/oozz.latin1");

// Start and end stop symbols
const SYMBOLS: &str = "[]";

// Visible character width. A character actually has an arbitrary width when
// including escape sequences, but this is the visible width
const CHAR_WIDTH: usize = 18;

// const INIT: &str = "[0;1;40;32m";

pub fn valid_chars(v: String) -> Result<(), String> {
    let re = Regex::new(r"[^a-zA-Z\s!\.]").unwrap();
    match re.captures(&v[..]) {
        None => return Ok(()),
        Some(cap) => Err(format!("Unsupported character: {}", &cap[0]))
    }
}

/// Function to parse character font, and extra characters. Split up input files
/// into hash map and use char it represents as a key
fn parse_string(input: &str, letters: &str, color: u8) -> HashMap<char, Vec<String>> {
    let mut map = HashMap::new();
    let mut lines: Vec<String> = {
        if color == 32 {
            input.lines().map(|l| String::from(l)).collect()
        } else {
            let green_re = Regex::new(r"32m").unwrap();
            input.lines()
                .map(|l| green_re.replace_all(l, &format!("{}m", color)[..]).into_owned())
                .collect()
        }
    };

    for c in letters.chars() {
        map.insert(c, lines.drain(..LETTER_HEIGHT).collect());
    }
    return map;
}

/// Parses the oozz character font.
///
/// Due to the way the program I use to draw ANSI art represents whitespace,
/// this function has to do some gymnastics to construct the output lines when
/// they are composed together.
///
/// In the resource file oozz.ans trailing whitespace is not included, which is
/// an issue then composing lengths of string based on arbitrary input.
/// Furthermore, whitespace that is included is represented using `Esc[ValueC`
/// where value is the amount the cursor is moved forward. Again an issue if you
/// are composing together sets of characters and trying to calculate a fixed
/// width based on what you can see.
///
/// This is solved by calculating a whitespace padding by counting missing
/// whitespace (trimmed away from end of line) taking into account the escaped
/// cursor movement.
///
/// Just wanted to add all this here so maybe this code makes sense down the
/// road.
fn parse_oozz(input: &str, color: u8) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = Vec::new();
    let mut padded: Vec<String> = Vec::new();

    // Create a regexp that captures all escape sequences in input
    // and one that captures cursor_forward escapes.
    let all_escape_re = Regex::new(r"\x1b[^m]*m").unwrap();
    let cursor_forward_re = Regex::new(r"(\x1b\[)([0-9]+)(C)").unwrap();
    let green_re = Regex::new(r"32m").unwrap();

    for line in input.lines() {

        // capture all cursor forward padding in input string
        let mut captured_padding = 0;
        for cap in cursor_forward_re.captures_iter(line) {
            captured_padding = captured_padding + &cap[2].parse::<usize>().expect("Failed to parse cursor_forward_re capture");
        }

        // remove all escapes from input, and count chars
        let trimmed = all_escape_re.replace_all(line, "");
        let count = trimmed.chars().count();

        // calculate padding based on a constant visible width (CHAR_WIDTH)
        // subtracting the count from above and the captured_padding from escape sequences
        let pad_count = CHAR_WIDTH - count - captured_padding;

        // construct a padded string that is prepended to the line from unprocessed input
        let pad: String = (0..pad_count).map(|_| " ").collect();
        let colorized: String = {
            if color == 32 {
                String::from(line)
            } else {
                green_re.replace_all(line, &format!("{}m", color)[..]).into_owned()
            }
        };
        padded.push(colorized + &pad[..]);
    }

    // finally split into characters and return
    while !padded.is_empty() {
        out.push(padded.drain(..OOZZ_HEIGHT).collect());
    }
    out
}

/// Choose some oozz randomly from a set based on input from user
fn choose_oozz(input: &str, oozz: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut rng = rand::thread_rng();
    let mut out = Vec::new();
    for _ in input.chars() {
        let chosen = rng.choose(&oozz).expect("Failed to randomly choose an oozz character from parsed");
        out.push(chosen.to_vec());
    }
    out
}

fn produce_chars(input: &str, color: u8) -> Result<Vec<String>, Box<Error>> {
    let chars = parse_string(CHARS, LETTERS, color);
    let extra = parse_string(EXTRA, SYMBOLS, color);
    let chars_start = extra.get(&'[').ok_or("Couldn't retrive start character from extra")?;
    let chars_stop = extra.get(&']').ok_or("Couldn't retrive end character from extra")?;

    let mut out = Vec::new();
    for n in 0..LETTER_HEIGHT {
        let mut line = chars_start[n].clone();
        for input_char in input.chars() {
            let output_char = chars.get(&input_char).ok_or("Failed to retrieve character from chars")?;
            line = line + &output_char[n][..];
        }
        line = line + &chars_stop[n][..];
        out.push(line)
    }
    Ok(out)
}

fn produce_oozz(input: &str, color: u8) -> Result<Vec<String>, Box<Error>> {
    let oozz = parse_oozz(OOZZ, color);
    let oozz_stop = "─┘";
    let oozz_start = "└─";
    let oozz = choose_oozz(&input, &oozz);

    let mut out = Vec::new();

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
        out.push(line);
    }
    Ok(out)
}

fn get_color_id(color: &str) -> Result<u8, String> {
    match color {
        "black" => Ok(30),
        "red" => Ok(31),
        "green" => Ok(32),
        "yellow" => Ok(33),
        "blue" => Ok(34),
        "magenta" => Ok(35),
        "cyan" => Ok(36),
        "white" => Ok(37),
        &_ => Err(format!("unable to match provided color: {}", &color))
    }
}

pub fn run(matches: clap::ArgMatches) -> Result<(), Box<Error>> {

    let values: Vec<&str> = matches.values_of("INPUT").unwrap().collect();
    let color = get_color_id(matches.value_of("color").unwrap_or("green"))?;
    let input = values.join(" ");

    for c in produce_chars(&input, color)? {
        println!("{}", c);
    }

    for o in produce_oozz(&input, color)? {
        println!("{}", o);
    }

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
