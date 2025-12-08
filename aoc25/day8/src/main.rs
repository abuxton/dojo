use std::error::Error;
use std::fs;

use day8::solve;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let input = fs::read_to_string(&path)?;
    let ans = solve(&input);
    println!("{}", ans);
    Ok(())
}
