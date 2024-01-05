use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

//const INPUT: &str = "puzzle_inputs/day04/sample.input";
const INPUT: &str = "puzzle_inputs/day04/puzzle.input";

pub fn main() -> Result<(), Error> {
    let input = File::open(INPUT)?;
    let buffered = BufReader::new(input);

    let mut buffer: Vec<String> = Vec::new();
    for line in buffered.lines() {
        let s = line?;
        buffer.push(s);
    }

    let puzzle_input = parse_lines(&buffer);

    let xs: Vec<(u32, u32)> = puzzle_input
        .iter()
        .map(|(id, w, c)| {
            let s1: HashSet<u32> = HashSet::from_iter(w.iter().cloned());
            let s2: HashSet<u32> = HashSet::from_iter(c.iter().cloned());
            let s = s1.intersection(&s2);
            (*id, s.count() as u32)
        })
        .collect();

    let total_pts: u32 = xs
        .iter()
        .map(|(_, pt)| {
            let base: u32 = 2;
            if *pt > 0 {
                base.pow(*pt - 1)
            } else {
                0
            }
        })
        .sum();

    println!("{:?}", total_pts);
    //assert_eq!(total_pts, 13);

    Ok(())
}

type T = (u32, Vec<u32>, Vec<u32>);

fn parse_lines(lines: &[String]) -> Vec<T> {
    let mut v = Vec::new();

    for line in lines {
        let xs: Vec<&str> = line.split(':').collect();
        let card_id: Vec<_> = xs[0].split(' ').filter(|s| !s.is_empty()).collect();
        let card_id: u32 = card_id[1].parse().unwrap_or_default();
        let rest: Vec<_> = xs[1].split('|').filter(|s| !s.is_empty()).collect();
        let winning_numbers: Vec<u32> = rest[0]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap_or_default())
            .collect();
        let card_numbers: Vec<u32> = rest[1]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap_or_default())
            .collect();
        v.push((card_id, winning_numbers, card_numbers));
    }
    v
}
