#![allow(dead_code)]

/// # --- Day 2: Cube Conundrum ---
///
///
/// You're launched high into the atmosphere! The apex of your trajectory
/// just barely reaches the surface of a large island floating in the sky.
/// You gently land in a fluffy pile of leaves. It's quite cold, but you
/// don't see much snow. An Elf runs over to greet you.
///
/// The Elf explains that you've arrived at Snow Island and apologizes for
/// the lack of snow. He'll be happy to explain the situation, but it's
/// a bit of a walk, so you have some time. They don't get many visitors
/// up here; would you like to play a game in the meantime?
///
/// As you walk, the Elf shows you a small bag and some cubes which are
/// either red, green, or blue. Each time you play this game, he will hide
/// a secret number of cubes of each color in the bag, and your goal is
/// to figure out information about the number of cubes.
///
/// To get information, once a bag has been loaded with cubes, the Elf will
/// reach into the bag, grab a handful of random cubes, show them to you,
/// and then put them back in the bag. He'll do this a few times per game.
///
/// You play several games and record the information from each game
/// (your puzzle input). Each game is listed with its ID number (like the
/// 11 in Game 11: ...) followed by a semicolon-separated list of subsets
/// of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).
///
/// For example, the record of a few games might look like this:
///
/// - Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
///
/// - Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
///
/// - Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
///
/// - Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
///
/// - Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
///
/// In game 1, three sets of cubes are revealed from the bag (and then put
/// back again). The first set is 3 blue cubes and 4 red cubes; the second
/// set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only
/// 2 green cubes.
///
///
/// The Elf would first like to know which games would have been possible if
/// the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
///
/// In the example above, games 1, 2, and 5 would have been possible if the
/// bag had been loaded with that configuration. However, game 3 would have
/// been impossible because at one point the Elf showed you 20 red cubes
/// at once; similarly, game 4 would also have been impossible because the
/// Elf showed you 15 blue cubes at once. If you add up the IDs of the games
/// that would have been possible, you get 8.
///
///
/// Determine which games would have been possible if the bag had been
/// loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes.
/// What is the sum of the IDs of those games?
///
///
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

const INPUT: &str = "src/days/day_2/puzzle_input";

pub fn main() -> Result<(), Error> {
    let input = File::open(INPUT)?;
    let buffered = BufReader::new(input);

    let mut games = Vec::new();
    for line in buffered.lines() {
        let s = line?;
        let g = Game::game_from_str(&s);
        games.push(g);
    }

    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    let id_sum: u32 = games
        .iter()
        .filter(|g| g.is_possible(&bag))
        .map(|g| g.0)
        .sum();

    // println!("{id_sum}");
    assert_eq!(id_sum, 2600);

    Ok(())
}

#[derive(Debug, Default)]
struct Draw {
    red: u32,
    blue: u32,
    green: u32,
}

impl Draw {
    fn new() -> Self {
        Self::default()
    }
}

impl Draw {
    /// parse input of the form: "10 red"
    fn set_cube_from_str(&mut self, input: &str) {
        let cube: Vec<&str> = input
            .split(' ')
            .filter(|s| !s.is_empty() && s.chars().all(char::is_alphanumeric))
            .collect();

        match cube.as_slice() {
            [n, "red"] => self.red = n.parse().unwrap_or(self.red),
            [n, "blue"] => self.blue = n.parse().unwrap_or(self.blue),
            [n, "green"] => self.green = n.parse().unwrap_or(self.green),
            _ => {
                println!("Unknown pattern: {input}");
            }
        }
    }
}

#[derive(Debug)]
struct Game(u32, Vec<Draw>);

impl Game {
    fn game_from_str(input: &str) -> Self {
        let tokens = tokenize(input, ':');
        if tokens.len() != 2 {
            panic!("Unknown pattern: {input}");
        }

        let id = tokenize(tokens[0], ' ');
        if id.len() != 2 || !id[1].chars().all(|c| c.is_ascii_digit()) {
            panic!("Unknown id part: {:?}", id);
        }

        let id = id[1].parse().unwrap_or_default();

        let cube_lists = tokenize(tokens[1], ';');

        let mut draws = Vec::new();
        for cube_list in cube_lists {
            let mut draw = Draw::new();

            for cube in tokenize(cube_list, ',') {
                draw.set_cube_from_str(cube);
            }

            draws.push(draw);
        }

        Self(id, draws)
    }

    fn is_possible(&self, bag: &Bag) -> bool {
        bag.is_possible(self)
    }
}

#[derive(Debug, Default)]
struct Bag {
    red: u32,
    blue: u32,
    green: u32,
}

impl Bag {
    fn is_possible(&self, game: &Game) -> bool {
        for draw in &game.1 {
            if self.red < draw.red || self.blue < draw.blue || self.green < draw.green {
                return false;
            }
        }

        true
    }
}

fn tokenize(input: &str, del: char) -> Vec<&str> {
    input.split(del).filter(|tok| !tok.is_empty()).collect()
}
