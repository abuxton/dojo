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
            println!("Fresh Products: {}", part1);
            println!("Total Unique IDs: {}", part2);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error solving puzzle: {}", e);
            Err(e)
        }
    }
}

/// Parse input and solve both parts:
/// - Part 1: count queries IN any range (fresh ingredients from available list)
/// - Part 2: count total IDs covered by all ranges (merge overlapping ranges)
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
    let mut ranges: Vec<(u64, u64)> = Vec::new();
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

        let a: u64 = parts[0].trim().parse()
            .map_err(|e| format!(
                "Invalid number '{}' in range on line {}: {}",
                parts[0].trim(),
                line_num + 1,
                e
            ))?;

        let b: u64 = parts[1].trim().parse()
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

    // Parse queries for Part 1
    let mut queries: Vec<u64> = Vec::new();
    let query_section_start = sections[0].lines().count() + 2;

    for (idx, line) in sections[1].lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let q: u64 = line.parse()
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

    // Part 1: count queries in ANY range (fresh from available list)
    let part1 = queries
        .iter()
        .filter(|&&q| ranges.iter().any(|&(a, b)| q >= a && q <= b))
        .count();

    // Part 2: count total unique IDs covered by all ranges
    let part2 = count_ids_in_ranges(&ranges)
        .try_into()
        .map_err(|_| "Part 2 result too large for usize")?;

    Ok((part1, part2))
}

/// Merge overlapping ranges and count total IDs covered.
/// Returns the count of unique IDs in all merged ranges.
fn count_ids_in_ranges(ranges: &[(u64, u64)]) -> u64 {
    if ranges.is_empty() {
        return 0;
    }

    // Sort ranges by start position
    let mut sorted = ranges.to_vec();
    sorted.sort_by_key(|r| r.0);

    // Merge overlapping ranges
    let mut merged: Vec<(u64, u64)> = Vec::new();
    let mut current = sorted[0];

    for &(a, b) in &sorted[1..] {
        if a <= current.1 + 1 {
            // Overlapping or adjacent - merge
            current.1 = current.1.max(b);
        } else {
            // Gap - save current and start new
            merged.push(current);
            current = (a, b);
        }
    }
    merged.push(current);

    // Count total IDs in merged ranges
    merged.iter().map(|(a, b)| b - a + 1).sum()
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
        assert_eq!(part2, 14); // total IDs: 3,4,5,10-20 = 14 IDs
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
        assert_eq!(part2, 10); // 1-10 = 10 IDs
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
        assert_eq!(part2, 11); // 10-20 = 11 IDs
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
        assert_eq!(part2, 11); // merged 5-15 = 11 IDs
    }

    #[test]
    fn non_overlapping_ranges() {
        let input = "\
1-5
10-15
20-25

3
";
        let (part1, part2) = solve(input).unwrap();
        assert_eq!(part1, 1);
        assert_eq!(part2, 17); // 5 + 6 + 6 = 17 IDs
    }

    #[test]
    fn adjacent_ranges_merge() {
        let input = "\
1-5
6-10

3
";
        let (_part1, part2) = solve(input).unwrap();
        assert_eq!(part2, 10); // 1-10 merged = 10 IDs
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
    fn handles_large_numbers() {
        let input = "\
263168346238540-263168346238550

263168346238545
999999999999999
";
        let (part1, part2) = solve(input).unwrap();
        assert_eq!(part1, 1); // one query in range
        assert_eq!(part2, 11); // 263168346238540-263168346238550 = 11 IDs
    }

    #[test]
    fn handles_empty_queries() {
        let input = "\
1-10

";
        assert!(solve(input).is_err());
    }
}

