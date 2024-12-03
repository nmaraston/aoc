mod solution;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Puzzle day number (1 - 24)
    day: u32,

    /// Puzzle part (1 or 2)
    part: u32,

    /// Path to puzzle input file
    input_file: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let solution = solution::get_solution(cli.day);

    let file = File::open(cli.input_file)?;
    let mut reader = BufReader::new(file);

    match cli.part {
        1 => println!("{}", solution.part_1(&mut reader)?),
        2 => println!("{}", solution.part_2(&mut reader)?),
        _ => panic!(
            "Illegal part number specified '{}'. Must be '1' or '2'",
            cli.part
        ),
    };

    Ok(())
}
