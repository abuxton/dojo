use std::error::Error;
use std::fs;

/// Read input file, print part1 and part2 results.
fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let input = fs::read_to_string(path)?;
    let part1 = solve(&input);
    let part2 = solve_part2(&input);
    println!("{}", part1);
    println!("{}", part2);
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
            let digits: Vec<u8> = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c as u8 - b'0')
                .collect();
            if digits.len() < 2 {
                return 0u64;
            }
            // suffix max approach for k=2
            let n = digits.len();
            let mut suffix_max = vec![0u8; n];
            suffix_max[n - 1] = digits[n - 1];
            for i in (0..n - 1).rev() {
                suffix_max[i] = digits[i].max(suffix_max[i + 1]);
            }
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

/// Compute the lexicographically largest subsequence of length `k` (digits preserved order)
/// and return its numeric value. Returns 0 if not enough digits.
fn max_subsequence_value(digits: &[u8], k: usize) -> u128 {
    if digits.len() < k || k == 0 {
        return 0;
    }
    let n = digits.len();
    let mut result: u128 = 0;
    let mut start = 0usize;
    for i in 0..k {
        // we must pick position in [start ..= n - (k - i)]
        let end = n - (k - i);
        let mut best_pos = start;
        let mut best_digit = digits[start];
        for pos in (start + 1)..=end {
            let d = digits[pos];
            if d > best_digit {
                best_digit = d;
                best_pos = pos;
                if best_digit == 9 { break; } // early exit
            }
        }
        result = result * 10 + (best_digit as u128);
        start = best_pos + 1;
    }
    result
}

/// Part 2: choose exactly 12 digits (preserve order) per line to form the largest 12-digit number,
/// then sum those numbers across lines. Returns u128 (sum can be large).
pub fn solve_part2(input: &str) -> u128 {
    const K: usize = 12;
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|line| {
            let digits: Vec<u8> = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c as u8 - b'0')
                .collect();
            max_subsequence_value(&digits, K)
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
        // "12a3" digits -> 1,2,3 -> best two-digit is 23
        // "9" -> not enough digits -> 0
        // "ab45" -> 4,5 -> 45
        assert_eq!(solve(input), 23 + 0 + 45);
    }

    #[test]
    fn part2_example_from_readme() {
        let input = "\
987654321111111
811111111111119
234234234234278
818181911112111
";
        // expected per README:
        // 987654321111 + 811111111119 + 434234234278 + 888911112111 = 3121910778619
        assert_eq!(solve_part2(input), 3_121_910_778_619u128);
    }

    #[test]
    fn max_subsequence_simple_cases() {
        let digits = b"123456789".iter().map(|b| b - b'0').collect::<Vec<u8>>();
        // choose k=2 -> best 89
        assert_eq!(max_subsequence_value(&digits, 2), 89u128);
        // choose k=3 -> 789
        assert_eq!(max_subsequence_value(&digits, 3), 789u128);

        let digits2 = b"998877".iter().map(|b| b - b'0').collect::<Vec<u8>>();
        // choose k=4 -> pick first four 9,9,8,8 -> 9988
        assert_eq!(max_subsequence_value(&digits2, 4), 9988u128);
    }
}
