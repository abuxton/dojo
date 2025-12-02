// use std::collections::HashSet;
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

// compute 10^n as Option<u128>
fn pow10(n: usize) -> Option<u128> {
    let mut v: u128 = 1;
    for _ in 0..n {
        v = v.checked_mul(10)?;
    }
    Some(v)
}

// integer ceil division
fn div_ceil(a: u128, b: u128) -> u128 {
    if b == 0 { 0 } else { (a + b - 1) / b }
}

// return divisors of n
fn divisors(n: usize) -> Vec<usize> {
    let mut ds = Vec::new();
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            ds.push(i);
            if i * i != n {
                ds.push(n / i);
            }
        }
        i += 1;
    }
    ds.sort_unstable();
    ds
}

// Möbius function μ(n)
fn mobius(n: usize) -> i32 {
    if n == 1 { return 1; }
    let mut x = n;
    let mut cnt = 0;
    let mut p = 2;
    while p * p <= x {
        if x % p == 0 {
            x /= p;
            cnt += 1;
            if x % p == 0 {
                return 0;
            }
        } else {
            p += 1 + (p & 1); // simple increment
        }
    }
    if x > 1 { cnt += 1; }
    if cnt % 2 == 0 { 1 } else { -1 }
}

/// Parse input and compute:
/// - part1: sum of numbers that are exactly two repetitions of the same k-digit block (r == 2)
/// - part2: sum of numbers that are repetitions of some k-digit block with r >= 2
pub fn solve(input: &str) -> Result<(u128, u128), Box<dyn Error>> {
    let line = input.lines().find(|l| !l.trim().is_empty()).ok_or("empty input")?;
    let mut total_p1: u128 = 0;
    let mut total_p2: u128 = 0;

    for token in line.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
        let mut parts = token.splitn(2, '-');
        let a: u128 = parts.next().ok_or("bad range")?.parse()?;
        let b: u128 = parts.next().ok_or("bad range")?.parse()?;
        if a > b { continue; }

        // maximum total digits we need to consider: stop when smallest number with that many digits > b
        let mut max_total_digits = 1usize;
        while let Some(p10) = pow10(max_total_digits) {
            if p10 - 1 > b { break; }
            max_total_digits += 1;
            if max_total_digits > 38 { break; } // u128 safety
        }

        // iterate primitive base length d
        for d in 1..=max_total_digits {
            let pow10_d = match pow10(d) { Some(v) => v, None => break };
            let x_min_allowed = pow10_d / 10;
            let x_max_allowed = pow10_d - 1;

            // quick check: minimal repeated value for r=2 using smallest d-digit x
            // denom_r2 = 10^{2d} -1 / (10^d -1) = 10^d + 1
            let pow10_2d = match pow10(2 * d) { Some(v) => v, None => break };
            let denom_r2 = (pow10_2d - 1) / (pow10_d - 1); // equals 10^d + 1
            if let Some(min_val_r2) = x_min_allowed.checked_mul(denom_r2) {
                if min_val_r2 > b {
                    // further d will only increase minimal values (since x_min_allowed grows), break d loop
                    break;
                }
            }

            // r = repetition count
            let mut r = 2usize;
            loop {
                // compute pow10^{d*r}
                let pow10_dr = match pow10(d * r) { Some(v) => v, None => break };
                // denom = (10^{d*r} - 1) / (10^d - 1)
                let denom = (pow10_dr - 1) / (pow10_d - 1);

                // minimal value for this d,r
                if let Some(min_val) = x_min_allowed.checked_mul(denom) {
                    if min_val > b {
                        break; // larger r will increase denom -> stop r loop
                    }
                } else {
                    break;
                }

                // base x range that produces values in [a,b]: ceil(a/denom) .. floor(b/denom)
                let lx = div_ceil(a, denom);
                let ux = b / denom;
                let lower = lx.max(x_min_allowed);
                let upper = ux.min(x_max_allowed);
                if lower <= upper {
                    // direct sum of x in [lower, upper] (safe arithmetic) for part1 when r == 2
                    let n = (upper - lower + 1) as u128;
                    let sum_x = if n % 2 == 0 {
                        match (n / 2).checked_mul(lower + upper) {
                            Some(v) => v,
                            None => { r += 1; continue; }
                        }
                    } else {
                        match ((lower + upper) / 2).checked_mul(n) {
                            Some(v) => v,
                            None => { r += 1; continue; }
                        }
                    };
                    if r == 2 {
                        // denom * sum_x (checked)
                        if let Some(add_p1) = (denom as u128).checked_mul(sum_x) {
                            total_p1 = total_p1.saturating_add(add_p1);
                        }
                    }

                    // compute sum of primitive base x in [lower, upper] using Möbius inversion for part2
                    let mut sum_primitive_x: i128 = 0;
                    for &e in divisors(d).iter() {
                        let mu = mobius(e) as i128;
                        if mu == 0 { continue; }
                        let de = d / e;
                        let pow10_de = match pow10(de) { Some(v) => v, None => continue };
                        let denom_me = (pow10_d - 1) / (pow10_de - 1); // M_e
                        let y_lower = div_ceil(lower, denom_me);
                        let y_upper = upper / denom_me;
                        let y_min_allowed = pow10_de / 10;
                        let y_max_allowed = pow10_de - 1;
                        let yl = y_lower.max(y_min_allowed);
                        let yu = y_upper.min(y_max_allowed);
                        if yl <= yu {
                            let n_y = (yu - yl + 1) as u128;
                            let sum_y = if n_y % 2 == 0 {
                                match (n_y / 2).checked_mul(yl + yu) {
                                    Some(v) => v,
                                    None => continue,
                                }
                            } else {
                                match ((yl + yu) / 2).checked_mul(n_y) {
                                    Some(v) => v,
                                    None => continue,
                                }
                            };
                            let contrib_u = match (denom_me as u128).checked_mul(sum_y) {
                                Some(v) => v,
                                None => continue,
                            };
                            if let Ok(contrib_i) = i128::try_from(contrib_u) {
                                sum_primitive_x = sum_primitive_x.saturating_add((mu as i128) * contrib_i);
                            } else {
                                let prev = if sum_primitive_x < 0 { 0u128 } else { sum_primitive_x as u128 };
                                if mu > 0 {
                                    sum_primitive_x = match (prev.saturating_add(contrib_u)).try_into() {
                                        Ok(v) => v,
                                        Err(_) => i128::MAX,
                                    };
                                } else {
                                    let sub = if prev > contrib_u { prev - contrib_u } else { 0 };
                                    sum_primitive_x = match sub.try_into() {
                                        Ok(v) => v,
                                        Err(_) => 0,
                                    };
                                }
                            }
                        }
                    } // end divisors
                    let sum_px = if sum_primitive_x < 0 { 0u128 } else { sum_primitive_x as u128 };
                    let add = denom.saturating_mul(sum_px);
                    total_p2 = total_p2.saturating_add(add);
                }

                r += 1;
            } // end r loop
        } // end d loop
    } // end tokens

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
		  // expected part1 value from README; adjust if different
        assert_eq!(p1, 1_227_775_554u128);
		  // expected part2 value from README; adjust if different calculation
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

