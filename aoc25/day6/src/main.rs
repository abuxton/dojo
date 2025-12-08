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
/// Within each problem, read columns right-to-left.
/// Each column provides one digit to each row's number (ones, tens, hundreds, ...).
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

    eprintln!("=== Part 2: Right-to-Left ===");

    // Identify problem boundaries (columns that are all spaces separate problems)
    let mut problem_starts = Vec::new();
    let mut in_problem = false;

    for col in 0..max_width {
        let is_empty_col = lines.iter().all(|l| {
            col >= l.len() || l.chars().nth(l.len().min(col)) == Some(' ')
        });

        if !is_empty_col && !in_problem {
            problem_starts.push(col);
            in_problem = true;
        } else if is_empty_col && in_problem {
            in_problem = false;
        }
    }

    let mut grand_total: u64 = 0;

    // Process each problem from right to left
    for (prob_idx, &start_col) in problem_starts.iter().rev().enumerate() {
        // Find end of this problem
        let mut end_col = start_col;
        while end_col < max_width && !lines.iter().all(|l| {
            end_col >= l.len() || l.chars().nth(end_col) == Some(' ')
        }) {
            end_col += 1;
        }
        end_col -= 1; // Back up to last non-empty column

        eprintln!("\nProblem {} (processing RTL, cols {}-{}):", prob_idx + 1, start_col, end_col);

        // Find operation (last row within this problem)
        let mut operation = String::new();
        for col in start_col..=end_col {
            if let Some(ch) = lines[last_line_idx].chars().nth(col) {
                if ch != ' ' {
                    operation.push(ch);
                }
            }
        }
        let operation = operation.trim();
        eprintln!("  Operation: '{}'", operation);

        // Build numbers for each row by reading columns right-to-left
        let mut row_numbers: Vec<String> = vec![String::new(); last_line_idx];

        eprintln!("  Reading columns right-to-left:");
        // Read columns right-to-left
        for col in (start_col..=end_col).rev() {
            // Skip if this column has the operation
            let has_operation = lines[last_line_idx]
                .chars()
                .nth(col)
                .map(|ch| ch != ' ')
                .unwrap_or(false);

            if has_operation {
                eprintln!("    Col {}: [operation column - skipped]", col);
                continue;
            }

            eprintln!("    Col {}:", col);
            // Each column provides one digit to each row
            for row in 0..last_line_idx {
                if let Some(ch) = lines[row].chars().nth(col) {
                    if ch != ' ' {
                        eprintln!("      Row {}: '{}' -> prepending to row_numbers[{}] = '{}'",
                            row, ch, row, row_numbers[row]);
                        // Prepend digit (reading RTL, so first column is least significant)
                        row_numbers[row].insert(0, ch);
                    }
                }
            }
        }

        eprintln!("  Numbers built from rows:");
        for (i, s) in row_numbers.iter().enumerate() {
            if !s.is_empty() {
                eprintln!("    Row {}: '{}'", i, s);
            }
        }

        // Parse numbers
        let numbers: Vec<u64> = row_numbers
            .iter()
            .filter_map(|s| {
                if s.is_empty() {
                    None
                } else {
                    s.parse::<u64>().ok()
                }
            })
            .collect();

        if numbers.is_empty() {
            eprintln!("  No numbers found - skipping");
            continue;
        }

        eprintln!("  Parsed numbers: {:?}", numbers);

        // Compute result
        let result = match operation {
            "*" => {
                let prod = numbers.iter().product::<u64>();
                eprintln!("  Calculation: {} = {}",
                    numbers.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" * "),
                    prod
                );
                prod
            }
            "+" => {
                let sum = numbers.iter().sum::<u64>();
                eprintln!("  Calculation: {} = {}",
                    numbers.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" + "),
                    sum
                );
                sum
            }
            _ => return Err(format!("Unknown operation: {}", operation).into()),
        };

        eprintln!("  Problem result: {}", result);
        grand_total += result;
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
        // Reading RTL: each row forms a number
        // Row 0: 5
        // Row 1: 3
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
