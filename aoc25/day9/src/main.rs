use std::error::Error;
use std::fs;

use day9::solve_part1_with_options;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_path = "input.txt".to_string();
    let mut visualize = false;

    for arg in std::env::args().skip(1) {
        if let Some(rest) = arg.strip_prefix("input=") {
            input_path = rest.to_string();
        } else if arg == "visualize=true" || arg == "viz=true" || arg == "--visualize" {
            visualize = true;
        } else {
            eprintln!("Unrecognized arg: {arg}");
        }
    }

    let input = fs::read_to_string(&input_path)?;
    let part1 = solve_part1_with_options(&input, visualize);

    println!("{}", part1);
    Ok(())
}
