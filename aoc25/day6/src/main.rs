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
/// Each column contains digits of a single number (top = most significant).
pub fn solve_part2(input: &str) -> Result<u64, Box<dyn Error>> {
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

    let last_line_idx = lines.len() - 1;

    // Identify problem boundaries by reading right-to-left
    // A column of all spaces separates problems
    let mut problem_boundaries = Vec::new();
    let mut in_problem = false;

    for col in (0..max_width).rev() {
        let is_empty_col = lines.iter().all(|l| {
            col >= l.len() || l.chars().nth(col) == Some(' ')
        });

        if !is_empty_col && !in_problem {
            problem_boundaries.push(col);
            in_problem = true;
        } else if is_empty_col && in_problem {
            problem_boundaries.push(col);
            in_problem = false;
        }
    }

    if in_problem {
        problem_boundaries.push(0);
    }

    // Reverse to process problems left-to-right
    problem_boundaries.reverse();

    let mut grand_total: u64 = 0;

    // Process each problem (defined by its boundaries)
    for i in (0..problem_boundaries.len()).step_by(2) {
        if i + 1 >= problem_boundaries.len() {
            break;
        }

        let start_col = problem_boundaries[i];
        let end_col = problem_boundaries[i + 1];

        // Get operation (last line in this column range)
        let operation = lines[last_line_idx]
            .chars()
            .skip(start_col)
            .take(end_col - start_col)
            .collect::<String>()
            .trim()
            .to_string();

        if operation.is_empty() {
            continue;
        }

        // Read columns right-to-left within this problem
        let mut numbers = Vec::new();
        let mut current_number_digits = Vec::new();

        for col in (start_col..end_col).rev() {
            // Check if this is a separator column (all spaces in data rows)
            let is_separator = (0..last_line_idx).all(|row| {
                col >= lines[row].len() || lines[row].chars().nth(col) == Some(' ')
            });

            if is_separator && !current_number_digits.is_empty() {
                // Convert digits to number - FIX: use join instead of collect
                let num_str: String = current_number_digits.join("");
                if let Ok(num) = num_str.parse::<u64>() {
                    numbers.push(num);
                }
                current_number_digits.clear();
            } else if !is_separator {
                // Collect digits from top to bottom
                let mut digit_str = String::new();
                for row in 0..last_line_idx {
                    if let Some(ch) = lines[row].chars().nth(col) {
                        if ch != ' ' {
                            digit_str.push(ch);
                        }
                    }
                }
                if !digit_str.is_empty() {
                    current_number_digits.push(digit_str);
                }
            }
        }

        // Add last number if exists - FIX: use join instead of collect
        if !current_number_digits.is_empty() {
            let num_str: String = current_number_digits.join("");
            if let Ok(num) = num_str.parse::<u64>() {
                numbers.push(num);
            }
        }

        if numbers.is_empty() {
            continue;
        }

        // Compute result
        let result = match operation.as_str() {
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
