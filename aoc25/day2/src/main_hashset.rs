use std::collections::HashSet;
use std::error::Error;
use std::fs;

const DEFAULT_INPUT_FILE: &str = "example_input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args().nth(1).unwrap_or_else(|| DEFAULT_INPUT_FILE.to_string());
    let input = fs::read_to_string(path)?;
    let (part1, part2) = solve(&input)?;
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}

/// Parse input and compute:
/// - part1: sum of numbers that are exactly two repetitions of the same k-digit block (r == 2)
/// - part2: sum of numbers that are repetitions of some k-digit block with r >= 2
pub fn solve(input: &str) -> Result<(u128, u128), Box<dyn Error>> {
    let line = input.lines().find(|l| !l.trim().is_empty()).ok_or("empty input")?;
    // collect unique values to avoid double-counting across different (k,r)
    let mut set_p1: HashSet<u128> = HashSet::new();
    let mut set_p2: HashSet<u128> = HashSet::new();

    for token in line.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
        let mut parts = token.splitn(2, '-');
        let a: u128 = parts.next().ok_or("bad range")?.parse()?;
        let b: u128 = parts.next().ok_or("bad range")?.parse()?;
        if a > b { continue; }

        // Iterate base length k = 1,2,...
        let mut pow10_k: u128 = 1; // 10^k, will multiply at loop start
        for _k in 1.. {
            pow10_k = match pow10_k.checked_mul(10) {
                Some(v) => v,
                None => break,
            };
            let x_min_allowed = pow10_k / 10;
            if x_min_allowed == 0 { continue; }

            // early exit if even smallest k-digit x for r=2 exceeds b
            if let Some(pow10_kr2) = pow10_k.checked_mul(pow10_k) {
                let denom_r2 = (pow10_kr2.saturating_sub(1)) / (pow10_k.saturating_sub(1));
                if let Some(min_val_r2) = x_min_allowed.checked_mul(denom_r2) {
                    if min_val_r2 > b { break; }
                }
            } else { break; }

            let mut pow10_kr = pow10_k;
            for r in 2.. {
                pow10_kr = match pow10_kr.checked_mul(pow10_k) {
                    Some(v) => v,
                    None => break,
                };

                let denom_num = match pow10_kr.checked_sub(1) { Some(v) => v, None => break };
                let denom_den = pow10_k - 1;
                if denom_den == 0 { break; }
                let denom = denom_num / denom_den;

                let min_val = match x_min_allowed.checked_mul(denom) {
                    Some(v) => v,
                    None => break,
                };
                if min_val > b { break; }

                let x_min = (a + denom - 1) / denom;
                let x_max = b / denom;
                let lower = x_min.max(x_min_allowed);
                let upper = x_max.min(pow10_k - 1);

                if lower <= upper {
                    // iterate x and insert actual values into sets to deduplicate across (k,r)
                    // For typical AoC inputs this iteration is small; adjust if needed for huge ranges.
                    for x in lower..=upper {
                        let val = x.saturating_mul(denom);
                        if r == 2 {
                            set_p1.insert(val);
                        }
                        set_p2.insert(val);
                    }
                }
            }
        }
    }

    let total_p1 = set_p1.into_iter().fold(0u128, |acc, v| acc.saturating_add(v));
    let total_p2 = set_p2.into_iter().fold(0u128, |acc, v| acc.saturating_add(v));
    Ok((total_p1, total_p2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_from_readme() {
        let input = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";
        let (p1, p2) = solve(input).unwrap();
        assert_eq!(p1, 1_227_775_554u128);
        // expected part2 value from README; adjust if different
        assert_eq!(p2, 4_174_379_265u128);
    }

    #[test]
    fn small_ranges() {
        let input = "11-22\n";
        let (p1, p2) = solve(input).unwrap();
        assert_eq!(p1, 33u128);
        assert_eq!(p2, 33u128);
    }

    #[test]
    fn large_k() {
        let input = "100100-100100,1212-1212\n"; // 100100 = 100 repeated, 1212 = 12 repeated
        let (p1, p2) = solve(input).unwrap();
        assert_eq!(p1, 100100u128 + 1212u128);
        assert_eq!(p2, 100100u128 + 1212u128);
    }
}

