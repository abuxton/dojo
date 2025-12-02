use std::error::Error;
use std::fs;

const DEFAULT_INPUT_FILE: &str = "example_input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args().nth(1).unwrap_or_else(|| DEFAULT_INPUT_FILE.to_string());
    let input = fs::read_to_string(path)?;
    let (part1, part2) = solve(&input);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}

/// Solve both parts:
/// - part1: count rotations whose final position is 0
/// - part2: count every click (during rotations, including the final click) that lands on 0
pub fn solve(input: &str) -> (usize, usize) {
    let mut pos: i64 = 50;
    let mut part1 = 0usize;
    let mut part2 = 0usize;

    for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let dir = line.chars().next().expect("empty line");
        let steps: i64 = line[1..].parse().expect("invalid rotation number");

        // Count hits during this rotation (clicks that land on 0)
        part2 += hits_in_rotation(pos.rem_euclid(100) as i64, steps, dir);

        // Apply rotation to update position
        pos = match dir {
            'L' | 'l' => (pos - steps).rem_euclid(100),
            'R' | 'r' => (pos + steps).rem_euclid(100),
            _ => panic!("invalid direction: {}", dir),
        };

        if pos == 0 {
            part1 += 1;
        }
    }

    (part1, part2)
}

/// Count how many k in 1..=steps satisfy (start + dir*k) % 100 == 0
fn hits_in_rotation(start: i64, steps: i64, dir: char) -> usize {
    if steps <= 0 {
        return 0;
    }
    // Normalize start into 0..99
    let s = start.rem_euclid(100);

    // first k (1-based) that hits 0
    let mut k_first = match dir {
        'R' | 'r' => (100 - s) % 100, // k such that (s + k) % 100 == 0
        'L' | 'l' => s % 100,         // k such that (s - k) % 100 == 0 -> k == s mod100
        _ => panic!("invalid direction: {}", dir),
    };
    if k_first == 0 {
        k_first = 100;
    }
    if k_first > steps {
        0
    } else {
        // additional full 100-step cycles within remaining steps
        1 + ((steps - k_first) / 100) as usize
    }
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
        let (p1, p2) = solve(input);
        assert_eq!(p1, 3);
        assert_eq!(p2, 6);
    }

    #[test]
    fn large_rotation_counts_multiple_hits() {
        // starting at 50, R1000 -> passes 0 ten times (k_first = 50 -> 50,150,...,950)
        let (p1, p2) = solve("R1000\n");
        // final position returns to 50 -> part1 = 0
        assert_eq!(p1, 0);
        assert_eq!(p2, 10);
    }
}
