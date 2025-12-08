use std::error::Error;
use std::fs;

use day8::solve;

fn main() -> Result<(), Box<dyn Error>> {
    // args: [0]=bin, [1]=input path (optional, default input.txt), [2]=k pairs (optional, default 1000)
    let mut args = std::env::args().skip(1);
    let path = args.next().unwrap_or_else(|| "input.txt".to_string());
    let k: usize = args
        .next()
        .map(|s| s.parse().expect("k must be a positive integer"))
        .unwrap_or(1000);

    let input = fs::read_to_string(&path)?;
    let ans = solve(&input, k);
    println!("{}", ans);
    Ok(())
}
