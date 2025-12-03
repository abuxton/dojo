use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let input = fs::read_to_string(path)?;
    let total = solve(&input);
    println!("{}", total);
    Ok(())
}

/// For each non-empty line, find the maximum two-digit number obtainable by
/// selecting two digits in order (i < j) and concatenating them (10*a + b).
/// Sum those maxima across lines.
pub fn solve(input: &str) -> u64 {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|line| {
            // collect digits only (skip whitespace / other chars)
            let digits: Vec<u8> = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c as u8 - b'0')
                .collect();
            if digits.len() < 2 {
                return 0u64;
            }
            let n = digits.len();
            // build suffix max to get best trailing digit for each position
            let mut suffix_max = vec![0u8; n];
            suffix_max[n - 1] = digits[n - 1];
            for i in (0..n - 1).rev() {
                suffix_max[i] = digits[i].max(suffix_max[i + 1]);
            }
            // compute best 10*digits[i] + best_following
            let mut best: u8 = 0;
            for i in 0..n - 1 {
                let ones = suffix_max[i + 1];
                let val = digits[i].saturating_mul(10).saturating_add(ones);
                if val > best {
                    best = val;
                }
            }
            best as u64
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_from_readme() {
        let input = "\
987654321111111
811111111111119
234234234234278
818181911112111
";
        assert_eq!(solve(input), 357);
    }

    #[test]
    fn handles_non_digits_and_short_lines() {
        let input = "\
12a3
9
ab45
";
        // lines: "12a3" -> digits 1,2,3 -> best 23 (choose 2 then 3)
        // "9" -> less than 2 digits -> 0
        // "ab45" -> digits 4,5 -> 45
        assert_eq!(solve(input), 23 + 0 + 45);
    }
}
