#![allow(unused_variables, unused_assignments)]

use std::collections::HashMap;
/// # --- Day 3: Gear Ratios ---
///
/// Starting time  : Tue Dec  5 04:52:15 PM IST 2023
///
/// Completion time: Wed Dec  6 08:28:36 AM IST 2023
///
///
/// You and the Elf eventually reach a gondola lift station; he says the
/// gondola lift will take you up to the water source, but this is as far
/// as he can bring you. You go inside.
///
/// It doesn't take long to find the gondolas, but there seems to be a
/// problem: they're not moving.
///
/// "Aaah!"
///
/// You turn around to see a slightly-greasy Elf with a wrench and a look of
/// surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't
/// working right now; it'll still be a while before I can fix it." You
/// offer to help.
///
/// The engineer explains that an engine part seems to be missing from the
/// engine, but nobody can figure out which one. If you can add up all the
/// part numbers in the engine schematic, it should be easy to work out
/// which part is missing.
///
/// The engine schematic (your puzzle input) consists of a visual
/// representation of the engine. There are lots of numbers and symbols you
///  don't really understand, but apparently any number adjacent to a symbol, even diagonally,
///  is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)
///
/// Here is an example engine schematic:
///
/// 467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..
///
/// In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right)
/// and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
///
/// Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?
///
/// ## Idea
///
/// Add 2 rows and 2 columns of '.' as boundaries. Now if a number is eclosed inside a rectangle of '.' is not a part number.
///
/// ............
/// .467..114...
/// ....*.......
/// ...35..633..
/// .......#....
/// .617*.......
/// ......+.58..
/// ...592......
/// .......755..
/// ....$.*.....
/// ..664.598...
/// ............
///
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

//const INPUT: &str = "puzzle_inputs/day03/sample_input";
const INPUT: &str = "puzzle_inputs/day03/puzzle_input";
const N: usize = 500;

const OUTPUT: &str = "part1.out";

pub fn main() -> Result<(), Error> {
    let input = File::open(INPUT)?;
    let buffered = BufReader::new(input);

    let mut matrix: [[char; N + 2]; N + 2] = [['.'; N + 2]; N + 2];

    let (mut r, mut c) = (1, 1);
    for line in buffered.lines() {
        let l = line?;

        for x in l.chars() {
            matrix[r][c] = x;
            c += 1;
        }
        c = 1;
        r += 1;
    }

    let mut indexes = Vec::new();

    for i in 1..N + 1 {
        let mut v = Vec::new();
        for j in 1..N + 1 {
            v.push((i, j));
        }
        indexes.push(v);
    }

    let mut num_positions = Vec::new();
    for row in indexes.iter() {
        let mut start = (0, 0);
        let mut end = (0, 0);

        let mut i = 0;
        loop {
            while i < row.len() && !get_char(&matrix, row[i]).is_ascii_digit() {
                i += 1;
            }

            if i >= row.len() {
                break;
            }
            start = row[i];

            while i < row.len() && get_char(&matrix, row[i]).is_ascii_digit() {
                i += 1;
            }
            end = row[i - 1];

            num_positions.push((start, end));
        }
    }

    let part_number_locations: Vec<U> = num_positions
        .iter()
        .filter(|(a, b)| is_part_number(&matrix, *a, *b))
        .cloned()
        .collect();

    // let sum_part_numbers: u32 = part_number_locations
    //     .iter()
    //     .map(|(a, b)| get_part_number(&matrix, *a, *b))
    //     .sum();
    //
    //assert_eq!(sum_part_numbers, 532428);

    let star_locations: Vec<P> = indexes
        .iter()
        .flatten()
        .filter(|p| get_char(&matrix, **p) == '*')
        .cloned()
        .collect();

    // for (start, end) in part_number_locations {
    //     println!("{:?}", get_adjacent_stars(&matrix, start, end));
    // }

    let xs: Vec<(P, u32)> = part_number_locations
        .iter()
        .flat_map(|(start, end)| {
            get_adjacent_stars(&matrix, *start, *end)
                .iter()
                .map(|p| (*p, get_part_number(&matrix,*start, *end)))
                .collect::<Vec<(P, u32)>>()
        })
        .collect();

    let mut hmap: HashMap<P, Vec<u32>> = HashMap::new();
    for (star, part_number) in xs {
        hmap.entry(star).and_modify(|v| v.push(part_number)).or_insert_with(|| vec![part_number]);
    }

    hmap.retain(|_, v| v.len() == 2);

    let sum_gear_ratios: u32 = hmap.values().map(|v| v[0] * v[1]).sum();
    println!("{:?}", sum_gear_ratios);
    //assert_eq!(sum_gear_ratios, 467835);

    Ok(())
}

fn get_adjacent_stars(matrix: &[[char; N + 2]; N + 2], start: P, end: P) -> Vec<P> {
    let bps = get_border_points(start, end);

    bps.iter()
        .filter(|p| get_char(matrix, **p) == '*')
        .cloned()
        .collect()
}

type P = (usize, usize);
type U = (P, P);

type T = (P, u32);

fn get_border_points(start: P, end: P) -> Vec<P> {
    let mut border = Vec::new();

    let (x, y1) = start;
    let (_, y2) = end;

    for x in [x - 1, x + 1] {
        for y in (y1 - 1)..(y2 + 2) {
            border.push((x, y));
        }
    }
    border.push((x, y1 - 1));
    border.push((x, y2 + 1));

    border
}

fn get_border_mappings(
    matrix: &[[char; N + 2]; N + 2],
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<T> {
    let mut mappings = Vec::new();

    let (x1, y1) = start;
    let (_, y2) = end;

    let mut checks = Vec::new();
    for x in [x1 - 1, x1 + 1] {
        for y in (y1 - 1)..(y2 + 2) {
            checks.push((x, y));
        }
    }
    checks.push((x1, y1 - 1));
    checks.push((x1, y2 + 1));

    for p in checks.iter() {
        mappings.push((*p, get_part_number(matrix, start, end)));
    }

    mappings
}

fn get_char(m: &[[char; N + 2]; N + 2], p: (usize, usize)) -> char {
    m[p.0][p.1]
}

fn is_part_number(m: &[[char; N + 2]; N + 2], start: (usize, usize), end: (usize, usize)) -> bool {
    let (x1, y1) = start;
    let (_, y2) = end;

    let mut checks = Vec::new();
    for x in [x1 - 1, x1 + 1] {
        for y in (y1 - 1)..(y2 + 2) {
            checks.push((x, y));
        }
    }
    checks.push((x1, y1 - 1));
    checks.push((x1, y2 + 1));

    for p in checks {
        let ch = get_char(m, p);
        if ch != '.' && !ch.is_ascii_digit() {
            return true;
        }
    }
    false
}

fn get_part_number(m: &[[char; N + 2]; N + 2], start: (usize, usize), end: (usize, usize)) -> u32 {
    if !is_part_number(m, start, end) {
        return 0;
    }
    let (x1, y1) = start;
    let (_, y2) = end;
    let mut s = String::new();
    for y in y1..y2 + 1 {
        //print!("({}, {}) => {} ", x1, y, get_char(m, (x1, y1)));

        let ch = get_char(m, (x1, y));
        s.push(ch);
    }
    s.parse().unwrap_or_default()
}
