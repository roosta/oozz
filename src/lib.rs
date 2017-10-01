use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    pub input: String,
}

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

// 1. read each for file
// 2. for each 18 chars, put string in vector
// 3. profit
pub fn run(config: Config) -> Result<(), Box<Error>>{

    let mut a = File::open("resources/chars/a.latin1")?;
    let mut b = File::open("resources/chars/b.latin1")?;

    let mut aa = String::new();
    let mut bb = String::new();
    a.read_to_string(&mut aa)?;
    b.read_to_string(&mut bb)?;


    // let mut iter = bb.chars().enumerate().map(|(i, c)| if i % 18 == 0 {

    // } else {
    //     c
    // });
//     fn titlecase_word(word: &str) -> String {
//         word.chars().enumerate()
//             .map(|(i, c)| if i == 0 { c.to_uppercase() } else { c.to_lowercase() })
//             .collect()
// }

    let mut result = String::new();
    for (i, c) in bb.chars().enumerate() {
        if i % 18 == 0 {
            result.push(c);
            result.push('\n');
        } else {
            result.push(c);
        }
    }

    println!("{}", result);

    // println!("{}{}\n", aa, bb);

    // for line in contents. {
    //     println!("{}", line);
    // }

    // println!("{}", &config.input);
    // let results = if config.case_sensitive {
    //     search(&config.query, &contents)
    // } else {
    //     search_case_insensitive(&config.query, &contents)
    // };


    // println!("{}", &config.input);
    // let results = if config.case_sensitive {
    //     search(&config.query, &contents)
    // } else {
    //     search_case_insensitive(&config.query, &contents)
    // };

    // for line in results {
    //     println!("{}", line);
    // }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
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
