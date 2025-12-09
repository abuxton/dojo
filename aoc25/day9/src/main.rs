use std::error::Error;
use std::fs;

use day9::solve_part1;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_path = "input.txt".to_string();

    for arg in std::env::args().skip(1) {
        if let Some(rest) = arg.strip_prefix("input=") {
            input_path = rest.to_string();
        } else {
            eprintln!("Unrecognized arg: {arg}");
        }
    }

    let input = fs::read_to_string(&input_path)?;
    let part1 = solve_part1(&input);

    println!("{}", part1);
    Ok(())
}
