use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());

    let input = fs::read_to_string(&input_path)?;
    let part1 = day10::solve_part1(&input)?;

    println!("{}", part1);
    Ok(())
}
