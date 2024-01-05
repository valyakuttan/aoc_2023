use std::collections::HashMap;

/// --- Day 8: Haunted Wasteland ---
///
/// You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching.
/// When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just
/// finished warning you about ghosts a few minutes ago.
///
/// One of the camel's pouches is labeled "maps" - sure enough, it's full of documents
/// (your puzzle input) about how to navigate the desert. At least, you're pretty sure that's what
/// they are; one of the documents contains a list of left/right instructions, and the rest of the
///  documents seem to describe some kind of network of labeled nodes.
///
/// It seems like you're meant to use the left/right instructions to navigate the network. Perhaps
/// if you have the camel follow the same instructions, you can escape the haunted wasteland!
///
/// After examining the maps for a bit, two nodes stick out: AAA and ZZZ. You feel like AAA is where
/// you are now, and you have to follow the left/right instructions until you reach ZZZ.
///
/// This format defines each node of the network individually. For example:
///
/// RL
///
/// AAA = (BBB, CCC)
/// BBB = (DDD, EEE)
//' CCC = (ZZZ, GGG)
//' DDD = (DDD, DDD)
/// EEE = (EEE, EEE)
/// GGG = (GGG, GGG)
/// ZZZ = (ZZZ, ZZZ)
///
/// Starting with AAA, you need to look up the next element based on the next left/right instruction in your
/// input. In this example, start with AAA and go right (R) by choosing the right element of AAA, CCC. Then,
/// L means to choose the left element of CCC, ZZZ. By following the left/right instructions, you reach ZZZ
/// in 2 steps.
///
/// Of course, you might not find ZZZ right away. If you run out of left/right instructions, repeat the whole
/// sequence of instructions as necessary: RL really means RLRLRLRLRLRLRLRL... and so on. For example, here is a
/// situation that takes 6 steps to reach ZZZ:
///
/// LLR
///
/// AAA = (BBB, BBB)
/// BBB = (AAA, ZZZ)
/// ZZZ = (ZZZ, ZZZ)
///
/// Starting at AAA, follow the left/right instructions. How many steps are required to reach ZZZ?
///
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

//const INPUT: &str = "puzzle_inputs/day08/sample1.input";
//const INPUT: &str = "puzzle_inputs/day08/sample2.input";
const INPUT: &str = "puzzle_inputs/day08/puzzle.input";

pub fn main() -> Result<(), Error> {
    let input = File::open(INPUT)?;
    let buffered = BufReader::new(input);

    let mut lines = Vec::new();
    for line in buffered.lines() {
        let s = line?;
        lines.push(s);
    }

    //let mut nodes = Vec::new();

    let ins = lines[0].trim().to_owned();
    lines.remove(0);
    lines.retain(|s| !s.is_empty());

    let table = parse_lines(&lines);
    let (node_map, left_transition, right_transition) = get_maps(&table);

    let start = node_map.get("AAA").unwrap();
    let goal = node_map.get("ZZZ").unwrap();
    let steps = steps_taken(*start, *goal, &ins, &left_transition, &right_transition);
    assert_eq!(steps, 20221);

    Ok(())
}

type Map = HashMap<u32, u32>;

fn steps_taken(start: u32, goal: u32, instr: &str, left: &Map, right: &Map) -> u32 {
    // println!("start: {} goal: {}, ins: {}", start, goal, instr);
    // println!();
    // println!("{:?}", left);
    // println!();
    // println!("{:?}", right);

    let mut count = 0;
    let directions = instr.chars().cycle();
    let mut current = start;

    let f = |c, s| next_state(s, c, left, right);

    for c in directions {
        current = f(c, current);
        count += 1;

        if current == goal {
            break;
        }
    }

    count
}

fn next_state(current: u32, turn: char, left: &Map, right: &Map) -> u32 {
    match turn {
        'L' => *left.get(&current).unwrap(),
        'R' => *right.get(&current).unwrap(),
        _ => panic!("unknown turn {turn}"),
    }
}

type MapTuple<'a> = (HashMap<&'a str, u32>, Map, Map);

fn get_maps<'a>(table: &'a [(&'a str, Vec<&'a str>)]) -> MapTuple<'a> {
    let node_str: Vec<&str> = table.iter().map(|(n, _)| *n).collect();
    let node_map: HashMap<&str, u32> = node_str
        .iter()
        .enumerate()
        .map(|(a, b)| (*b, a as u32))
        .collect();

    let f = |s, t| (*node_map.get(s).unwrap(), *node_map.get(t).unwrap());

    let left_map: HashMap<_, _> = table.iter().map(|(n, v)| f(*n, v[0])).collect();
    let right_map: HashMap<_, _> = table.iter().map(|(n, v)| f(*n, v[1])).collect();

    (node_map, left_map, right_map)
}

fn parse_lines(lines: &[String]) -> Vec<(&str, Vec<&str>)> {
    let mut buffer: Vec<(&str, Vec<&str>)> = Vec::new();
    for line in lines {
        let xs: Vec<&str> = line.split('=').filter(|s| !s.is_empty()).collect();

        let start = xs[0].trim();
        let next: Vec<&str> = xs[1]
            .split(&[' ', ',', '(', ')'])
            .filter(|s| !s.is_empty())
            .map(|s| s.trim())
            .collect();

        buffer.push((start, next));
    }
    buffer
}
