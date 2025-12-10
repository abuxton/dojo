use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    let input_path = args.get(1).map(|s| s.as_str()).unwrap_or("input.txt");
    let part = args.get(2).map(|s| s.as_str()).unwrap_or("1");

    let input = fs::read_to_string(input_path)?;

    match part {
        "1" => {
            let result = day10::solve_part1(&input)?;
            println!("Part 1: {}", result);
        }
        "2" => {
            let result = day10::solve_part2(&input)?;
            println!("Part 2: {}", result);
        }
        _ => {
            eprintln!("Usage: {} [input_file] [part (1|2)]", args[0]);
            std::process::exit(1);
        }
    }

    Ok(())
}
