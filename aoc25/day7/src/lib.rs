use std::collections::{HashSet, VecDeque};

/// Part 1: count how many times beams split (classical).
pub fn solve_part1(input: &str) -> u64 {
    let grid: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
    let rows = grid.len();
    let cols = grid.first().map(|r| r.len()).unwrap_or(0);
    let (sr, sc) = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| row.iter().position(|&c| c == b'S').map(|c| (r, c)))
        .expect("missing S");

    let mut splits: u64 = 0;
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    let mut split_seen = HashSet::new();

    if sr + 1 < rows {
        q.push_back((sr + 1, sc));
        seen.insert((sr + 1, sc));
    }

    while let Some((mut r, c)) = q.pop_front() {
        while r < rows {
            if grid[r][c] == b'^' {
                if split_seen.insert((r, c)) {
                    splits += 1;
                    if c > 0 && seen.insert((r, c - 1)) {
                        q.push_back((r, c - 1));
                    }
                    if c + 1 < cols && seen.insert((r, c + 1)) {
                        q.push_back((r, c + 1));
                    }
                }
                break;
            }
            r += 1;
        }
    }

    splits
}

/// Part 2: count timelines in the quantum case (many-worlds).
pub fn solve_part2(input: &str) -> u128 {
    let grid: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
    let rows = grid.len();
    let cols = grid.first().map(|r| r.len()).unwrap_or(0);
    if rows == 0 || cols == 0 {
        return 0;
    }
    let (sr, sc) = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| row.iter().position(|&c| c == b'S').map(|c| (r, c)))
        .expect("missing S");

    let mut counts = vec![vec![0u128; cols]; rows];
    let mut queue = VecDeque::new();
    let mut in_queue = HashSet::new();
    let mut terminated: u128 = 0;

    if sr + 1 >= rows {
        return 0;
    }
    counts[sr + 1][sc] = 1;
    queue.push_back((sr + 1, sc));
    in_queue.insert((sr + 1, sc));

    while let Some((r, c)) = queue.pop_front() {
        in_queue.remove(&(r, c));
        let ways = counts[r][c];
        if ways == 0 {
            continue;
        }

        match grid[r][c] {
            b'^' => {
                if c > 0 {
                    counts[r][c - 1] += ways;
                    if in_queue.insert((r, c - 1)) {
                        queue.push_back((r, c - 1));
                    }
                } else {
                    terminated += ways;
                }
                if c + 1 < cols {
                    counts[r][c + 1] += ways;
                    if in_queue.insert((r, c + 1)) {
                        queue.push_back((r, c + 1));
                    }
                } else {
                    terminated += ways;
                }
            }
            _ => {
                if r + 1 < rows {
                    counts[r + 1][c] += ways;
                    if in_queue.insert((r + 1, c)) {
                        queue.push_back((r + 1, c));
                    }
                } else {
                    terminated += ways;
                }
            }
        }

        counts[r][c] = 0;
    }

    terminated
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
    fn example_part1_splits() {
        assert_eq!(solve_part1(EXAMPLE), 21);
    }

    #[test]
    fn example_part2_timelines() {
        assert_eq!(solve_part2(EXAMPLE), 40);
    }
}
