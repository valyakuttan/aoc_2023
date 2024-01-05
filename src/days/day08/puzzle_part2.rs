use std::collections::{HashMap, HashSet};

/// --- Part Two ---
///
/// The sandstorm is upon you and you aren't any closer to escaping the wasteland. You had the camel
/// follow the instructions, but you've barely left your starting position. It's going to take
/// significantly more steps to escape!
///
/// What if the map isn't for people - what if the map is for ghosts? Are ghosts even bound by the
/// laws of spacetime? Only one way to find out.
///
/// After examining the maps a bit longer, your attention is drawn to a curious fact: the number
/// of nodes with names ending in A is equal to the number ending in Z! If you were a ghost, you'd
/// probably just start at every node that ends with A and follow all of the paths at the same
/// time until they all simultaneously end up at nodes that end with Z.
///
/// For example:
///
/// LR
///
/// 11A = (11B, XXX)
/// 11B = (XXX, 11Z)
/// 11Z = (11B, XXX)
/// 22A = (22B, XXX)
/// 22B = (22C, 22C)
/// 22C = (22Z, 22Z)
/// 22Z = (22B, 22B)
/// XXX = (XXX, XXX)
///
/// Here, there are two starting nodes, 11A and 22A (because they both end with A).
/// As you follow each left/right instruction, use that instruction to simultaneously
/// navigate away from both nodes you're currently on. Repeat this process until all
/// of the nodes you're currently on end with Z. (If only some of the nodes you're on
/// end with Z, they act like any other node and you continue as normal.) In this
/// example, you would proceed as follows:
///
///     Step 0: You are at 11A and 22A.
///     Step 1: You choose all of the left paths, leading you to 11B and 22B.
///     Step 2: You choose all of the right paths, leading you to 11Z and 22C.
///     Step 3: You choose all of the left paths, leading you to 11B and 22Z.
///     Step 4: You choose all of the right paths, leading you to 11Z and 22B.
///     Step 5: You choose all of the left paths, leading you to 11B and 22C.
///     Step 6: You choose all of the right paths, leading you to 11Z and 22Z.
///
/// So, in this example, you end up entirely on nodes that end in Z after 6 steps.
///
/// Simultaneously start on every node that ends with A. How many steps does it take
/// before you're only on nodes that end with Z?
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

//const INPUT: &str = "puzzle_inputs/day08/sample1.input";
//const INPUT: &str = "puzzle_inputs/day08/sample2.input";
const INPUT: &str = "puzzle_inputs/day08/sample3.input";

//const INPUT: &str = "puzzle_inputs/day08/puzzle.input";

type Map = HashMap<u32, u32>;
type Set = HashSet<u32>;

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

    // let start = node_map.get("AAA").unwrap();
    // let goal = node_map.get("ZZZ").unwrap();

    let rev_node_map: HashMap<u32, &str> = node_map.iter().map(|(n, l)| (*l, *n)).collect();
    let start: Vec<_> = node_map
        .iter()
        .filter(|(s, _)| s.ends_with('A'))
        .map(|(s, _)| *node_map.get(s).unwrap())
        .collect();

    // println!("{:?}", node_map);
    // println!("{:?}", left_transition);
    // println!("{:?}", right_transition);

    //println!("{:?}", start);
    //println!("{}", node_map.len());
    //let start = Vec::from([*node_map.get("AAA").unwrap()]);

    let steps = steps_taken(
        &start,
        &ins,
        &rev_node_map,
        &left_transition,
        &right_transition,
    );

    println!("Steps: {}", steps);
    assert_eq!(steps, 20221);

    Ok(())
}

fn steps_taken(
    start: &[u32],
    instr: &str,
    rev_node_map: &HashMap<u32, &str>,
    left: &Map,
    right: &Map,
) -> u32 {
    // println!("start: {} goal: {}, ins: {}", start, goal, instr);
    // println!();
    // println!("{:?}", left);
    // println!();
    // println!("{:?}", right);

    let mut count = 0;
    let directions = instr.chars().cycle();

    let mut current: Set = start.iter().cloned().collect();

    let goal_reached = |v: &Vec<u32>| {
        v.iter().all(|n| {
            let label = *rev_node_map.get(n).unwrap();
            label.ends_with('Z')
        })
    };

    for turn in directions {
        let v: Vec<u32> = current.drain().collect();

        if goal_reached(&v) {
            break;
        }
        count += 1;

        for node in v.iter() {
            current.insert(next_node(*node, turn, left, right));
        }

        if current.is_empty() {
            panic!("Current is empty");
        }
    }
    count
}

fn next_node(current_node: u32, turn: char, left: &Map, right: &Map) -> u32 {
    match turn {
        'L' => *left.get(&current_node).unwrap(),
        'R' => *right.get(&current_node).unwrap(),
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

    let left_map: HashMap<u32, u32> = table.iter().map(|(n, v)| f(*n, v[0])).collect();
    let right_map: HashMap<u32, u32> = table.iter().map(|(n, v)| f(*n, v[1])).collect();

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
