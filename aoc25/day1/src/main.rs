use std::error::Error;
use std::fs;

const DEFAULT_INPUT_FILE: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args().nth(1).unwrap_or_else(||  DEFAULT_INPUT_FILE.to_string());
    let input = fs::read_to_string(path)?;
    let count = solve(&input);
    println!("{}", count);
    Ok(())
}

/// Returns the number of times the dial points at 0 after a rotation sequence.
pub fn solve(input: &str) -> usize {
    let mut pos: i32 = 50;
    let mut zero_count = 0usize;

    for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let dir = line.chars().next().expect("empty line");
        let steps: i32 = line[1..].parse().expect("invalid rotation number");
        pos = match dir {
            'L' | 'l' => (pos - steps).rem_euclid(100),
            'R' | 'r' => (pos + steps).rem_euclid(100),
            _ => panic!("invalid direction: {}", dir),
        };
        if pos == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        assert_eq!(solve(input), 3);
    }
}
