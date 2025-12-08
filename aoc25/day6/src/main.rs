use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let input = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read {}: {}", path, e))?;

    match solve_part1(&input) {
        Ok(total1) => println!("{}", total1),
        Err(e) => {
            eprintln!("Error solving Part 1: {}", e);
            return Err(e);
        }
    }

    match solve_part2(&input) {
        Ok(total2) => println!("{}", total2),
        Err(e) => {
            eprintln!("Error solving Part 2: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

/// Part 1: Parse problems left-to-right.
/// Numbers are read as standard left-to-right integers.
pub fn solve_part1(input: &str) -> Result<u64, Box<dyn Error>> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return Err("Empty input".into());
    }

    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Pad all lines to same width
    let lines: Vec<String> = lines
        .iter()
        .map(|l| format!("{:<width$}", l, width = max_width))
        .collect();

    eprintln!("\n=== Part 1: Left-to-Right ===");

    // Identify problem boundaries (columns that are all spaces)
    let mut problem_starts = Vec::new();
    let mut in_problem = false;

    for col in 0..max_width {
        let is_empty_col = lines.iter().all(|l| {
            col >= l.len() || l.chars().nth(col) == Some(' ')
        });

        if !is_empty_col && !in_problem {
            problem_starts.push(col);
            in_problem = true;
        } else if is_empty_col && in_problem {
            in_problem = false;
        }
    }

    // Extract and solve each problem
    let mut grand_total: u64 = 0;
    let last_line_idx = lines.len() - 1;

    for (prob_num, &start_col) in problem_starts.iter().enumerate() {
        // Find end of this problem (next empty column or end of input)
        let mut end_col = start_col;
        while end_col < max_width && !lines.iter().all(|l| {
            end_col >= l.len() || l.chars().nth(end_col) == Some(' ')
        }) {
            end_col += 1;
        }

        // Extract problem text
        let problem_lines: Vec<String> = lines
            .iter()
            .map(|l| {
                let substring = if end_col <= l.len() {
                    &l[start_col..end_col]
                } else {
                    &l[start_col..]
                };
                substring.trim_end().to_string()
            })
            .collect();

        // Last line is the operation
        let operation = problem_lines[last_line_idx].trim();

        // Extract numbers from other lines (left-to-right)
        let mut numbers = Vec::new();
        for i in 0..last_line_idx {
            let line = &problem_lines[i];
            if let Ok(num) = line.trim().parse::<u64>() {
                numbers.push(num);
            }
        }

        if numbers.is_empty() {
            continue;
        }

        // Compute result based on operation
        let result = match operation {
            "*" => {
                let prod = numbers.iter().product::<u64>();
                eprintln!("Problem {} (cols {}-{}): {} = {}",
                    prob_num + 1,
                    start_col,
                    end_col - 1,
                    numbers.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" * "),
                    prod
                );
                prod
            }
            "+" => {
                let sum = numbers.iter().sum::<u64>();
                eprintln!("Problem {} (cols {}-{}): {} = {}",
                    prob_num + 1,
                    start_col,
                    end_col - 1,
                    numbers.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" + "),
                    sum
                );
                sum
            }
            _ => {
                return Err(format!("Unknown operation: {}", operation).into());
            }
        };

        grand_total += result;
    }

    eprintln!("Part 1 Total: {}\n", grand_total);
    Ok(grand_total)
}

/// Part 2: Parse problems right-to-left.
/// Each PROBLEM is anchored by an operator (`+` or `*`) in the bottom row.
/// The problem spans every non-blank column adjacent to that operator (to its
/// left and right until a fully-blank column is found).
/// Each COLUMN (excluding the bottom operator row) forms ONE number by reading
/// its digits top-to-bottom. Problems are evaluated right-to-left.
pub fn solve_part2(input: &str) -> Result<u64, Box<dyn Error>> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return Err("Empty input".into());
    }

    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let last_row = lines.len() - 1;

    // Pad all lines to the same width
    let grid: Vec<Vec<u8>> = lines
        .iter()
        .map(|l| {
            let mut v = l.as_bytes().to_vec();
            v.resize(max_width, b' ');
            v
        })
        .collect();

    let is_blank_col = |col: usize, g: &Vec<Vec<u8>>| -> bool {
        g.iter().all(|row| row[col] == b' ')
    };

    // Find all operator columns (non-space in last row)
    let mut ops: Vec<(usize, u8)> = Vec::new();
    for col in 0..max_width {
        let ch = grid[last_row][col];
        if ch != b' ' {
            ops.push((col, ch));
        }
    }

    let mut grand_total: u64 = 0;

    // Process problems right-to-left
    for (problem_idx, &(op_col, op_ch)) in ops.iter().rev().enumerate() {
        // Expand left until a blank column
        let mut start = op_col;
        while start > 0 && !is_blank_col(start - 1, &grid) {
            start -= 1;
        }
        // Expand right until a blank column
        let mut end = op_col;
        while end + 1 < max_width && !is_blank_col(end + 1, &grid) {
            end += 1;
        }

        eprintln!("\nProblem {}: cols {}..{} (op '{}' at col {})", problem_idx + 1, start, end, op_ch as char, op_col);

        let mut numbers = Vec::new();

        // Read columns right-to-left within this problem
        for col in (start..=end).rev() {
            // Build number from this column (top-to-bottom, skipping bottom op row)
            let mut digits = Vec::new();
            for row in 0..last_row {
                let ch = grid[row][col];
                if ch != b' ' {
                    digits.push(ch as char);
                }
            }

            if digits.is_empty() {
                continue;
            }

            let num_str: String = digits.into_iter().collect();
            if let Ok(num) = num_str.parse::<u64>() {
                eprintln!("  Col {} -> {}", col, num);
                numbers.push(num);
            }
        }

        if numbers.is_empty() {
            eprintln!("  (no numbers found)");
            continue;
        }

        // Compute
        let result = match op_ch {
            b'+' => {
                let sum: u64 = numbers.iter().sum();
                eprintln!("  Calc: {} = {}", numbers.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" + "), sum);
                sum
            }
            b'*' => {
                let prod: u64 = numbers.iter().product();
                eprintln!("  Calc: {} = {}", numbers.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" * "), prod);
                prod
            }
            _ => return Err(format!("Unknown operation: {}", op_ch as char).into()),
        };

        grand_total += result;
        eprintln!("  Problem result: {}", result);
        eprintln!("  Running total: {}", grand_total);
    }

    eprintln!("\nPart 2 Total: {}\n", grand_total);
    Ok(grand_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_from_readme() {
        let input = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";
        let total = solve_part1(input).unwrap();
        // 123*45*6 = 33210
        // 328+64+98 = 490
        // 51*387*215 = 4243455
        // 64+23+314 = 401
        // Total: 4277556
        assert_eq!(total, 4277556);
    }

    #[test]
    fn part2_example_from_readme() {
        let input = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";
        let total = solve_part2(input).unwrap();
        // Rightmost: 4 + 431 + 623 = 1058
        // Next: 175 * 581 * 32 = 3253600
        // Next: 8 + 248 + 369 = 625
        // Leftmost: 356 * 24 * 1 = 8544
        // Total: 3263827
        assert_eq!(total, 3263827);
    }

    #[test]
    fn part1_single_problem_add() {
        let input = "\
5
3
+";
        let total = solve_part1(input).unwrap();
        assert_eq!(total, 8);
    }

    #[test]
    fn part1_single_problem_multiply() {
        let input = "\
4
5
*";
        let total = solve_part1(input).unwrap();
        assert_eq!(total, 20);
    }

    #[test]
    fn part1_two_problems() {
        let input = "\
2 3
4 4
* +";
        let total = solve_part1(input).unwrap();
        // 2*4 = 8
        // 3+4 = 7
        // Total: 15
        assert_eq!(total, 15);
    }

    #[test]
    fn part2_single_column_add() {
        let input = "\
5
3
+";
        let total = solve_part2(input).unwrap();
        // Single column forms one number from top-to-bottom: "53"
        // 53 = 53
        assert_eq!(total, 53);
    }

    #[test]
    fn part2_single_column_multiply() {
        let input = "\
4
5
*";
        let total = solve_part2(input).unwrap();
        // Single column forms one number from top-to-bottom: "45"
        // 45 = 45
        assert_eq!(total, 45);
    }

    #[test]
    fn part1_three_numbers_multiply() {
        let input = "\
2
3
4
*";
        let total = solve_part1(input).unwrap();
        // 2*3*4 = 24
        assert_eq!(total, 24);
    }

    #[test]
    fn part1_three_numbers_add() {
        let input = "\
10
20
30
+";
        let total = solve_part1(input).unwrap();
        // 10+20+30 = 60
        assert_eq!(total, 60);
    }
}
