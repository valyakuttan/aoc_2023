#![allow(dead_code)]

// # --- Part Two ---
///
/// Your calculation isn't quite right. It looks like some of the digits are
/// actually spelled out with letters: one, two, three, four, five, six,
/// seven, eight, and nine also count as valid "digits".
///
/// Equipped with this new information, you now need to find the real first
/// and last digit on each line. For example:
///
/// two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen
///
/// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and
/// 76. Adding these together produces 281.
///
/// What is the sum of all of the calibration values?
///
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

const INPUT: &str = "src/days/day_1/puzzle_input";

pub fn main() -> Result<(), Error> {
    let input = File::open(INPUT)?;
    let buffered = BufReader::new(input);

    let trans = |s: String| recover_value(&word2num(&s));
    let f = |line: Result<String, Error>| line.map(trans);

    let sum: u32 = buffered
        .lines()
        .map(f)
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();

    assert_eq!(sum, 54581);
    Ok(())
}

fn word2num(txt: &str) -> String {
    let replacement = [
        ("zero", "0o"),
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "4"),
        ("five", "5e"),
        ("six", "6"),
        ("seven", "7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    let mut line = String::from(txt);

    for (word, rep) in replacement.iter() {
        line = line.replace(word, rep);
    }

    line.to_owned()
}

#[rustfmt::skip]
fn recover_value(s: &str) -> u32 {
    let s = String::from(s);
    let v = s.chars()
    .filter(|c| c.is_numeric())
    .map(|c| c.to_digit(10).unwrap())
    .collect::<Vec<_>>();
    
    let n = v.len();
    v[0] * 10 + v[n - 1]
}
