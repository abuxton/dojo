use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let input = fs::read_to_string(path)?;
    let (part1, part2) = solve(&input)?;
    println!("{}", part1);
    println!("{}", part2);
    Ok(())
}

/// Parse input and solve both parts:
/// - Part 1: count queries NOT in any range
/// - Part 2: count queries in ANY range
pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let sections: Vec<&str> = input.split("\n\n").collect();
    if sections.len() != 2 {
        return Err("Expected two sections separated by blank line".into());
    }

    // Parse ranges
    let ranges: Vec<(u32, u32)> = sections[0]
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.trim().split('-').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid range format: {}", line).into());
            }
            let a: u32 = parts[0].parse()?;
            let b: u32 = parts[1].parse()?;
            Ok((a, b))
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    // Parse queries
    let queries: Vec<u32> = sections[1]
        .lines()
        .map(|line| line.trim().parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    // Part 1: count queries NOT in any range
    let part1 = queries
        .iter()
        .filter(|&&q| !ranges.iter().any(|&(a, b)| q >= a && q <= b))
        .count();

    // Part 2: count queries in ANY range
    let part2 = queries
        .iter()
        .filter(|&&q| ranges.iter().any(|&(a, b)| q >= a && q <= b))
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
        assert_eq!(part1, 2); // 1 and 32 not in any range
        assert_eq!(part2, 4); // 5, 8, 11, 17 in ranges
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
        assert_eq!(part1, 0);
        assert_eq!(part2, 3);
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
        assert_eq!(part1, 3);
        assert_eq!(part2, 0);
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
        assert_eq!(part1, 1); // 20 not in range
        assert_eq!(part2, 2); // 7, 12 in ranges
    }
}
