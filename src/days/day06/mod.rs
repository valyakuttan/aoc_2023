mod puzzle;

use std::io::Error;

pub fn main() -> Result<(), Error> {
    puzzle::main();

    Ok(())
}
