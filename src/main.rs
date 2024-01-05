mod days;

use std::io::Error;

pub fn main() -> Result<(), Error> {
    days::main()?;

    Ok(())
}
