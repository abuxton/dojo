use std::error::Error;
use std::fs;

use day9::{solve_part1_with_options, solve_part2_with_options};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_path = "input.txt".to_string();
    let mut part = 1u8;
    let mut visualize = false;

    for arg in std::env::args().skip(1) {
        if arg == "--" {
            continue;
        } else if let Some(rest) = arg.strip_prefix("input=") {
            input_path = rest.to_string();
        } else if let Some(rest) = arg.strip_prefix("part=") {
            part = rest.parse().expect("part must be 1 or 2");
        } else if arg == "visualize=true" || arg == "viz=true" || arg == "--visualize" {
            visualize = true;
        } else {
            eprintln!("Unrecognized arg: {arg}");
        }
    }

    let input = fs::read_to_string(&input_path)?;
    let ans = match part {
        1 => solve_part1_with_options(&input, visualize),
        2 => solve_part2_with_options(&input, visualize),
        _ => panic!("part must be 1 or 2"),
    };

    println!("{ans}");
    Ok(())
}
