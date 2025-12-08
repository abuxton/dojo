use std::error::Error;
use std::fs;

// Pull in the library as a local module so names resolve in the bin target.
mod lib;
use crate::lib::{solve_part1, solve_part2};

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let input = fs::read_to_string(&path)?;
    let p1 = solve_part1(&input);
    let p2 = solve_part2(&input);
    println!("{}", p1);
    println!("{}", p2);
    Ok(())
}
