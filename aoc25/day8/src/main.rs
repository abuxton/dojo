use std::error::Error;
use std::fs;

use day8::solve;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_path = "input.txt".to_string();
    let mut pair_limit: usize = 1000;

    for arg in std::env::args().skip(1) {
        if let Some(rest) = arg.strip_prefix("input=") {
            input_path = rest.to_string();
        } else if let Some(rest) = arg.strip_prefix("k=") {
            pair_limit = rest.parse().expect("k must be a positive integer");
        } else {
            eprintln!("Unrecognized arg: {arg}");
        }
    }

    let input = fs::read_to_string(&input_path)?;
    let ans = solve(&input, pair_limit);
    println!("{}", ans);
    Ok(())
}
