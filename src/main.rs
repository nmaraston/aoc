use std::result::Result;
use std::error::Error;

use clap::{App, Arg, value_t};

mod solution;
use crate::solution::Solution;

fn main() -> Result<(), Box<Error>>  {
    let pkg_version = env!("CARGO_PKG_VERSION");
    let pkg_author = env!("CARGO_PKG_AUTHORS");

    let app_matches = App::new("aoc")
        .version(pkg_version)
        .author(pkg_author)
        .arg(
            Arg::with_name("DAY")
                .required(true)
                .index(1))
        .arg(
            Arg::with_name("PART")
                .required(true)
                .index(2))
        .get_matches();

    let day = value_t!(app_matches.value_of("DAY"), u32).unwrap();
    let part = value_t!(app_matches.value_of("PART"), u32).unwrap();

    let solution = match day {
        1 => solution::DAY1,
        _ => panic!("No solution implemented for given day {}", day),
    };

    match part {
        1 => println!("{}", solution.part_1()),
        2 => println!("{}", solution.part_2()),
        _ => panic!("Illegal part number specified '{}'. Must be '1' or '2'", part),
    };

    Ok(())
}
