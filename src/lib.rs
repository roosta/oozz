extern crate rand;
extern crate regex;
extern crate clap;
extern crate term_size;

#[macro_use] extern crate lazy_static;

use regex::Regex;
use std::error::Error;
use std::collections::HashMap;
// use rand::Rng;
use rand::{Rng, SeedableRng, StdRng};

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

/// validate input characters, ensure they are part of the allowed set
pub fn valid_chars(v: String) -> Result<(), String> {
    lazy_static! {
        static ref VALID_RE: Regex = Regex::new(r"[^a-zA-Z\s!\.]").unwrap();
    }
    match VALID_RE.captures(&v[..]) {
        None => Ok(()),
        Some(cap) => Err(format!("Unsupported character: {}", &cap[0]))
    }
}

/// locate green color as a regex match and replace with supplied color arg
fn colorize (line: &str, color: u8, bold: bool) -> String {
    let escape = format!("{};{}m", bold as u8, color);
    line.replace("32m", &escape)
}

/// At the start of each input file, there is appended a prelude of sorts, that
/// sets various properties for the following content. This prelude is
/// reoccurring and we only really need one so its trimmed away from all input
/// and added later
fn trim_prelude (line: &str) -> String {
    line.replace("\x1b[0;1;40;32m", "")
}

/// create custom prelude based on color choice,
fn create_prelude(color: u8, bold: bool) -> String {
    let b = if bold { "1;" } else { "" };
    format!("[0;{}{}m", b, color)
}

/// Function to parse character font, and extra characters. Split up input files
/// into hash map and use char it represents as a key
fn parse_string(input: &str, letters: &str, color: u8, bold: bool) -> HashMap<char, Vec<String>> {
    let mut map = HashMap::new();

    let mut lines: Vec<String> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            if i == 0 {
                colorize(&trim_prelude(l), color, bold)
            } else {
                colorize(l, color, bold)
            }
        })
        .collect();

    for c in letters.chars() {
        map.insert(c, lines.drain(..LETTER_HEIGHT).collect());
    }
    map
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
fn parse_oozz(input: &str) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = Vec::new();
    let mut padded: Vec<String> = Vec::new();

    // Create a regexp that captures all escape sequences in input
    // and one that captures cursor_forward escapes.
    let all_escape_re = Regex::new(r"(\x1b[^m]*m|\x1b\[\d+C)").unwrap();
    let cursor_forward_re = Regex::new(r"(\x1b\[)([0-9]+)(C)").unwrap();

    for line in input.lines() {

        // capture all cursor forward padding in input string
        let mut captured_padding = 0;
        for cap in cursor_forward_re.captures_iter(line) {
            captured_padding += &cap[2].parse::<usize>().expect("Failed to parse cursor_forward_re capture");
        }

        // remove all escapes from input, and count chars
        let trimmed = all_escape_re.replace_all(line, "");
        let count = trimmed.chars().count();

        // calculate padding based on a constant visible width (CHAR_WIDTH)
        // subtracting the count from above and the captured_padding from escape sequences
        let pad_count = CHAR_WIDTH - count - captured_padding;

        // construct a padded string that is prepended to the line from unprocessed input
        let pad = if pad_count > 0 {
            format!("\x1b[{}C", pad_count)
        } else {
            String::from("")
        };

        padded.push(String::from(line) + &pad[..]);
    }

    // finally split into characters and return
    while !padded.is_empty() {
        out.push(padded.drain(..OOZZ_HEIGHT).collect());
    }
    out
}

/// Choose some oozz randomly from a set based on input from user
fn choose_oozz(input: &str, oozz: &[Vec<String>]) -> Result<Vec<Vec<String>>, Box<Error>> {
    // let mut rng = rand::weak_rng();
    let seed: Vec<usize> = input.chars().map(|c| c as usize).collect();
    let mut rng: StdRng = SeedableRng::from_seed(&seed[..]);
    let mut out = Vec::new();
    for _ in input.chars() {
        let chosen = rng.choose(oozz).ok_or("Failed to randomly choose an oozz character from parsed")?;
        out.push(chosen.to_vec());
    }
    Ok(out)
}

/// Produces the final char output, composed from user input string.
/// Add start and end stops to output, so that its borders are completed
fn produce_chars(input: &str, color: u8, bold: bool) -> Result<Vec<String>, Box<Error>> {
    let chars = parse_string(CHARS, LETTERS, color, bold);
    let extra = parse_string(EXTRA, SYMBOLS, color, bold);
    let chars_start = extra.get(&'[').ok_or("Couldn't retrive start character from extra")?;
    let chars_stop = extra.get(&']').ok_or("Couldn't retrive end character from extra")?;

    let mut out = Vec::new();
    for n in 0..LETTER_HEIGHT {
        let mut line = chars_start[n].clone();
        for input_char in input.chars().enumerate() {
            let (i, c) = input_char;
            if i == 0 && n == 0 {
                line = [create_prelude(color, bold), line].concat();
            }
            let output_char = chars.get(&c).ok_or("Failed to retrieve character from chars")?;
            line += &output_char[n][..];
        }
        line += &chars_stop[n][..];
        out.push(line)
    }
    Ok(out)
}

fn produce_oozz(input: &str) -> Result<Vec<String>, Box<Error>> {
    let oozz = parse_oozz(OOZZ);
    let oozz_stop = "─┘";
    let oozz_start = "└─";
    let oozz = choose_oozz(input, &oozz)?;
    let mut out = Vec::new();

    for n in 0..OOZZ_HEIGHT {
        let mut line = String::new();
        if n == 0 {
            line += oozz_start;
        } else {
            line += "  ";
        }
        for input_char in input.chars().enumerate() {
            let (i, _) = input_char;
            let output_char = oozz.get(i).ok_or("Failed to retrieve character from oozz")?;
            line += &output_char[n][..];
        }
        if n == 0 {
            line += oozz_stop;
        } else {
            line += "  ";
        }
        out.push(line);
    }
    Ok(out)
}

fn get_color_id(color: &str) -> Result<u8, String> {
    match color {
        "black"   => Ok(30),
        "red"     => Ok(31),
        "green"   => Ok(32),
        "yellow"  => Ok(33),
        "blue"    => Ok(34),
        "magenta" => Ok(35),
        "cyan"    => Ok(36),
        "white"   => Ok(37),
        &_        => Err(format!("unable to match provided color: {}", &color))
    }
}

pub fn run(input: &str, color: &str, bold: bool, center: bool) -> Result<(), Box<Error>> {
    let color = get_color_id(color)?;
    let chars = produce_chars(&input, color, bold)?;
    let oozz = produce_oozz(&input)?;

    if center {
        let (width, _) = term_size::dimensions().ok_or("Failed to get terminal dimensions")?;
        let out_width = CHAR_WIDTH * input.chars().count();
        let padding = (width - out_width - 4) / 2;
        for c in chars {
            println!("\x1b[{}C{}", padding, c)
        }
        for o in oozz {
            println!("\x1b[{}C{}", padding, o)
        }
    } else {
        for c in chars {println!("{}", c);}
        for o in oozz {println!("{}", o);}
    };

    Ok(())
}

#[cfg(test)]
mod test;
