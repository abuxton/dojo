use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let input = fs::read_to_string(&path)?;
    println!("{}", solve(&input));
    Ok(())
}

/// Simulate tachyon beams and count how many times they are split.
pub fn solve(input: &str) -> u64 {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect();

    let rows = grid.len();
    let cols = grid.first().map(|r| r.len()).unwrap_or(0);

    // Find S
    let (sr, sc) = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| row.iter().position(|&c| c == b'S').map(|c| (r, c)))
        .unwrap();

    let mut splits: u64 = 0;
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();

    // Beam starts just below S
    if sr + 1 < rows {
        q.push_back((sr + 1, sc));
        seen.insert((sr + 1, sc));
    }

    while let Some((mut r, c)) = q.pop_front() {
        while r < rows {
            let cell = grid[r][c];
            if cell == b'^' {
                splits += 1;
                // emit left
                if c > 0 && seen.insert((r, c - 1)) {
                    q.push_back((r, c - 1));
                }
                // emit right
                if c + 1 < cols && seen.insert((r, c + 1)) {
                    q.push_back((r, c + 1));
                }
                break; // original beam stops at splitter
            }
            r += 1;
        }
    }

    splits
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ".......S.......\n\
                           ...............\n\
                           .......^.......\n\
                           ...............\n\
                           ......^.^......\n\
                           ...............\n\
                           .....^.^.^.....\n\
                           ...............\n\
                           ....^.^...^....\n\
                           ...............\n\
                           ...^.^...^.^...\n\
                           ...............\n\
                           ..^...^.....^..\n\
                           ...............\n\
                           .^.^.^.^.^...^.\n\
                           ...............";

    #[test]
    fn example_splits() {
        assert_eq!(solve(EXAMPLE), 21);
    }
}
