#![allow(dead_code)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

use std::io::Error;

pub fn main() -> Result<(), Error> {
    day09::main();
    //let _ = day05::main();

    Ok(())
}
