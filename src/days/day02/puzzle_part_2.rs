/// --- Part Two ---
///
/// The Elf says they've stopped producing snow because they aren't getting
/// any water! He isn't sure why the water stopped; however, he can show
/// you how to get to the water source to check it out for yourself.
/// It's just up ahead!
///
/// As you continue your walk, the Elf poses a second question: in each
/// game you played, what is the fewest number of cubes of each color
/// that could have been in the bag to make the game possible?
///
///
/// Again consider the example games from earlier:
///
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
///
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
///
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
///
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
///
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
///
/// In game 1, the game could have been played with as few as 4 red, 2 green,
/// and 6 blue cubes. If any color had even one fewer cube, the game would
/// have been impossible.
///
/// Game 2 could have been played with a minimum of 1 red, 3 green, and
/// 4 blue cubes.
///
/// Game 3 must have been played with at least 20 red, 13 green, and
/// 6 blue cubes.
///
/// Game 4 required at least 14 red, 3 green, and 15 blue cubes.
///
/// Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes
/// in the bag.
///
/// The power of a set of cubes is equal to the numbers of red, green, and
/// blue cubes multiplied together. The power of the minimum set of cubes
/// in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36,
/// respectively. Adding up these five powers produces the sum 2286.
///
/// For each game, find the minimum set of cubes that must have been
/// present. What is the sum of the power of these sets?
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

    let sum_of_powers: u32 = games
        .iter()
        .map(|g| g.min_set_of_cubes())
        .map(|b| b.power())
        .sum();

    assert_eq!(sum_of_powers, 86036);

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
    fn min_set_of_cubes(&self) -> Bag {
        let mut bag = Bag::default();

        for game in self.1.iter() {
            if bag.red < game.red {
                bag.red = game.red;
            }

            if bag.green < game.green {
                bag.green = game.green;
            }

            if bag.blue < game.blue {
                bag.blue = game.blue;
            }
        }

        bag
    }

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
    green: u32,
    blue: u32,
}

impl Bag {
    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }

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
