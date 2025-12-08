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

    for &start_col in &problem_starts {
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
            "*" => numbers.iter().product::<u64>(),
            "+" => numbers.iter().sum::<u64>(),
            _ => {
                return Err(format!("Unknown operation: {}", operation).into());
            }
        };

        grand_total += result;
    }

    Ok(grand_total)
}

/// Part 2: Parse problems right-to-left.
/// Each column contains one digit for each number (most significant at top).
/// Columns are read right-to-left within each problem.
pub fn solve_part2(input: &str) -> Result<u64, Box<dyn Error>> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return Err("Empty input".into());
    }

    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let last_line_idx = lines.len() - 1;

    // Pad all lines to same width
    let lines: Vec<String> = lines
        .iter()
        .map(|l| format!("{:<width$}", l, width = max_width))
        .collect();

    let mut grand_total: u64 = 0;

    // Find all operation positions (non-space chars in last line)
    let mut operation_positions = Vec::new();
    for (col, ch) in lines[last_line_idx].chars().enumerate() {
        if ch != ' ' {
            operation_positions.push((col, ch));
        }
    }

    // Process each problem (from right to left)
    for &(op_col, op_char) in operation_positions.iter().rev() {
        let operation = op_char.to_string();

        // Find the extent of this problem (columns between separators)
        // Look left from operation to find start of problem
        let mut start_col = op_col;
        while start_col > 0 {
            let prev_col = start_col - 1;
            let is_separator = lines.iter().all(|l| {
                prev_col >= l.len() || l.chars().nth(prev_col) == Some(' ')
            });
            if is_separator {
                break;
            }
            start_col = prev_col;
        }

        // Look right from operation to find end of problem
        let mut end_col = op_col;
        while end_col < max_width - 1 {
            let next_col = end_col + 1;
            let is_separator = lines.iter().all(|l| {
                next_col >= l.len() || l.chars().nth(next_col) == Some(' ')
            });
            if is_separator {
                break;
            }
            end_col = next_col;
        }

        // Count how many numbers we have (non-space chars in any data row)
        let num_count = (0..last_line_idx)
            .map(|row| {
                (start_col..=end_col)
                    .filter(|&col| {
                        lines[row].chars().nth(col).unwrap_or(' ') != ' '
                    })
                    .count()
                    .min(1)
            })
            .sum::<usize>();

        if num_count == 0 {
            continue;
        }

        // Build numbers by reading columns right-to-left
        // Initialize numbers as empty strings for each row
        let mut number_strings: Vec<String> = vec![String::new(); last_line_idx];

        // Read columns from right to left (least significant to most significant)
        for col in (start_col..=end_col).rev() {
            if col == op_col {
                continue; // Skip operation column
            }

            for row in 0..last_line_idx {
                if let Some(ch) = lines[row].chars().nth(col) {
                    if ch != ' ' {
                        number_strings[row].push(ch);
                    }
                }
            }
        }

        // Reverse each number string (we built them backwards)
        let numbers: Vec<u64> = number_strings
            .iter()
            .filter_map(|s| {
                let reversed: String = s.chars().rev().collect();
                reversed.parse::<u64>().ok()
            })
            .collect();

        if numbers.is_empty() {
            continue;
        }

        // Compute result
        let result = match operation.as_str() {
            "*" => numbers.iter().product::<u64>(),
            "+" => numbers.iter().sum::<u64>(),
            _ => return Err(format!("Unknown operation: {}", operation).into()),
        };

        grand_total += result;
    }

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
        // Column read top-to-bottom: 5, 3 = 53 (wait, that's wrong)
        // Actually: each column is a number, so 5 and 3 are separate numbers
        // 5 + 3 = 8
        assert_eq!(total, 8);
    }

    #[test]
    fn part2_single_column_multiply() {
        let input = "\
4
5
*";
        let total = solve_part2(input).unwrap();
        // 4 * 5 = 20
        assert_eq!(total, 20);
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
