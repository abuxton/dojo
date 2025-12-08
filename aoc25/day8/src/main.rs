use std::error::Error;
use std::fs;

use day8::{solve_part1, solve_part1_with_limit, solve_part2};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_path = "input.txt".to_string();
    let mut pair_limit: usize = 1000;
    let mut use_custom_limit = false;

    for arg in std::env::args().skip(1) {
        if let Some(rest) = arg.strip_prefix("input=") {
            input_path = rest.to_string();
        } else if let Some(rest) = arg.strip_prefix("pair_limit=") {
            pair_limit = rest.parse().expect("pair_limit must be a positive integer");
            use_custom_limit = true;
        } else {
            eprintln!("Unrecognized arg: {arg}");
        }
    }

    let input = fs::read_to_string(&input_path)?;
    let part1 = if use_custom_limit {
        solve_part1_with_limit(&input, pair_limit)
    } else {
        solve_part1(&input)
    };
    let part2 = solve_part2(&input);

    println!("{part1}");
    println!("{part2}");
    Ok(())
}
