use std::io::Error;
pub mod card;
pub mod hand;
pub mod card_part2;
pub mod hand_part2;

mod puzzle_part1;
mod puzzle_part2;

pub fn main() -> Result<(), Error> {
    puzzle_part2::main()?;
    //hand_part2::main();

    Ok(())
}
