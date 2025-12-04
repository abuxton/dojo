use std::error::Error;
use std::fs;

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

/// Count rolls of paper (@) that have fewer than 4 adjacent rolls in 8 directions.
pub fn solve(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    if grid.is_empty() {
        return 0;
    }
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '@' {
                continue;
            }
            // count neighbors that are '@'
            let neighbor_rolls = count_neighbors(&grid, r, c);
            if neighbor_rolls < 4 {
                count += 1;
            }
        }
    }
    count
}

/// Part 2: repeatedly remove all accessible rolls until no more can be removed.
/// Returns the total number of rolls removed.
pub fn solve_part2(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    if grid.is_empty() {
        return 0;
    }

    let mut total_removed = 0;

    loop {
        let accessible = find_accessible_positions(&grid);
        if accessible.is_empty() {
            break;
        }

        // Remove all accessible rolls
        for (r, c) in &accessible {
            grid[*r][*c] = '.';
        }

        total_removed += accessible.len();
    }

    total_removed
}

/// Find all positions with '@' that have fewer than 4 '@' neighbors.
fn find_accessible_positions(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let rows = grid.len();
    if rows == 0 {
        return Vec::new();
    }
    let cols = grid[0].len();
    let mut accessible = Vec::new();

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' {
                let neighbor_rolls = count_neighbors(grid, r, c);
                if neighbor_rolls < 4 {
                    accessible.push((r, c));
                }
            }
        }
    }
    // println!("{:?}", grid);
	// println!("{} accessables", accessible.len());
    accessible
}

/// Count how many of the 8 neighbors of position (r, c) contain '@'.
fn count_neighbors(grid: &[Vec<char>], r: usize, c: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                if grid[nr as usize][nc as usize] == '@' {
                    count += 1;
                }
            }
        }
    }
    // println!("{:?}", grid);
	// println!("{} neighbors", count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_from_readme_part1() {
        let input = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
        assert_eq!(solve(input), 13);
    }

    #[test]
    fn example_from_readme_part2() {
        let input = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
        assert_eq!(solve_part2(input), 43);
    }

    #[test]
    fn single_roll_accessible() {
        let input = "@\n";
        // single roll, 0 neighbors -> accessible
        assert_eq!(solve(input), 1);
        assert_eq!(solve_part2(input), 1);
    }

    #[test]
    fn fully_surrounded_not_accessible() {
        let input = "\
@@@
@@@
@@@
";
        // corners: 3 neighbors each (4 corners) -> accessible
        assert_eq!(solve(input), 4);
        // Part 2: remove 4 corners, then 4 edges become accessible (now 3 neighbors), then center
        assert_eq!(solve_part2(input), 9);
    }

    #[test]
    fn part2_iterative_removal() {
        // Simple 2x2 grid - all should be removable
        let input = "\
@@
@@
";
        // All 4 have 3 neighbors initially -> all accessible
        assert_eq!(solve(input), 4);
        assert_eq!(solve_part2(input), 4);
    }
}
