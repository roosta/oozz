extern crate oozz;

#[test]
fn no_args() {
    let expected: Vec<&str> = include_str!("no_args.txt").lines().collect();
    let result = oozz::run("test", "green", false, false, 0).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn yellow_bold() {
    let expected: Vec<&str> = include_str!("yellow_bold.txt").lines().collect();
    let result = oozz::run("_livid", "yellow", true, false, 0).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn blue_centered() {
    let expected: Vec<&str> = include_str!("blue_centered.txt").lines().collect();
    let result = oozz::run("'hello'", "blue", false, true, 159).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn all_args() {
    let expected: Vec<&str> = include_str!("all_args.txt").lines().collect();
    let result = oozz::run("./!1234567890", "magenta", true, true, 319).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn dollar_quote() {
    let expected: Vec<&str> = include_str!("dollar_quote.txt").lines().collect();
    let result = oozz::run("\"$\"", "green", false, false, 159).unwrap();
    assert_eq!(result, expected);
}
