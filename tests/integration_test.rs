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
    let result = oozz::run("test", "yellow", true, false, 0).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn blue_centered() {
    let expected: Vec<&str> = include_str!("blue_centered.txt").lines().collect();
    let result = oozz::run("test", "blue", false, true, 159).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn all_args() {
    let expected: Vec<&str> = include_str!("all_args.txt").lines().collect();
    let result = oozz::run("test", "magenta", true, true, 159).unwrap();
    assert_eq!(result, expected);
}
