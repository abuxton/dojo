use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let input = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read {}: {}", path, e))?;

    match solve(&input) {
        Ok((part1, part2)) => {
            println!("{}", part1);
            println!("{}", part2);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error solving puzzle: {}", e);
            Err(e)
        }
    }
}

/// Parse input and solve both parts:
/// - Part 1: count queries IN any range (fresh ingredients)
/// - Part 2: count queries NOT in any range (spoiled ingredients)
pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let sections: Vec<&str> = input.split("\n\n").collect();
    if sections.len() != 2 {
        return Err(format!(
            "Expected two sections separated by blank line, found {} sections",
            sections.len()
        )
        .into());
    }

    // Parse ranges with better error messages
    let mut ranges: Vec<(u32, u32)> = Vec::new();
    for (line_num, line) in sections[0].lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('-').collect();
        if parts.len() != 2 {
            return Err(format!(
                "Invalid range format on line {}: '{}' (expected 'a-b')",
                line_num + 1,
                line
            )
            .into());
        }

        let a: u32 = parts[0].trim().parse()
            .map_err(|e| format!(
                "Invalid number '{}' in range on line {}: {}",
                parts[0].trim(),
                line_num + 1,
                e
            ))?;

        let b: u32 = parts[1].trim().parse()
            .map_err(|e| format!(
                "Invalid number '{}' in range on line {}: {}",
                parts[1].trim(),
                line_num + 1,
                e
            ))?;

        if a > b {
            return Err(format!(
                "Invalid range on line {}: {} > {} (start must be <= end)",
                line_num + 1,
                a,
                b
            )
            .into());
        }

        ranges.push((a, b));
    }

    // Parse queries with better error messages
    let mut queries: Vec<u32> = Vec::new();
    let query_section_start = sections[0].lines().count() + 2; // +2 for blank line

    for (idx, line) in sections[1].lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let q: u32 = line.parse()
            .map_err(|e| format!(
                "Invalid query number '{}' on line {}: {}",
                line,
                query_section_start + idx,
                e
            ))?;

        queries.push(q);
    }

    if queries.is_empty() {
        return Err("No valid query numbers found in input".into());
    }

    // Part 1: count queries in ANY range (fresh)
    let part1 = queries
        .iter()
        .filter(|&&q| ranges.iter().any(|&(a, b)| q >= a && q <= b))
        .count();

    // Part 2: count queries NOT in any range (spoiled)
    let part2 = queries
        .iter()
        .filter(|&&q| !ranges.iter().any(|&(a, b)| q >= a && q <= b))
        .count();

    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_from_readme() {
        let input = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        let (part1, part2) = solve(input).unwrap();
        assert_eq!(part1, 3); // 5, 11, 17 are fresh (in ranges)
        assert_eq!(part2, 3); // 1, 8, 32 are spoiled (not in ranges)
    }

    #[test]
    fn all_queries_in_range() {
        let input = "\
1-10

3
5
7
";
        let (part1, part2) = solve(input).unwrap();
        assert_eq!(part1, 3); // all fresh
        assert_eq!(part2, 0); // none spoiled
    }

    #[test]
    fn no_queries_in_range() {
        let input = "\
10-20

1
2
30
";
        let (part1, part2) = solve(input).unwrap();
        assert_eq!(part1, 0); // none fresh
        assert_eq!(part2, 3); // all spoiled
    }

    #[test]
    fn overlapping_ranges() {
        let input = "\
5-10
8-15

7
12
20
";
        let (part1, part2) = solve(input).unwrap();
        assert_eq!(part1, 2); // 7, 12 are fresh
        assert_eq!(part2, 1); // 20 is spoiled
    }

    #[test]
    fn handles_invalid_range_format() {
        let input = "\
not-a-range

5
";
        assert!(solve(input).is_err());
    }

    #[test]
    fn handles_reversed_range() {
        let input = "\
10-5

7
";
        assert!(solve(input).is_err());
    }

    #[test]
    fn handles_overflow_numbers() {
        let input = "\
99999999999999999999-100

5
";
        assert!(solve(input).is_err());
    }

    #[test]
    fn handles_empty_queries() {
        let input = "\
1-10

";
        assert!(solve(input).is_err());
    }
}
