use crate::parser::Machine;
use std::error::Error;


pub fn parse_input(input: &str) -> Result<Vec<Machine>, Box<dyn Error>> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(parse_machine)
        .collect()
}

fn parse_machine(line: &str) -> Result<Machine, Box<dyn Error>> {
    // Extract target state from [...]
    let target_start = line.find('[').ok_or("Missing [ for target state")?;
    let target_end = line.find(']').ok_or("Missing ] for target state")?;
    let target_str = &line[target_start + 1..target_end];
    let target: Vec<bool> = target_str.chars().map(|c| c == '#').collect();

    // Extract buttons from (...)
    let mut buttons = Vec::new();
    let mut pos = target_end + 1;

    while let Some(start) = line[pos..].find('(') {
        let start = pos + start;
        let end = line[start..].find(')').ok_or("Missing ) for button")? + start;

        let button_str = &line[start + 1..end];
        let indices: Vec<usize> = button_str
            .split(',')
            .map(|s| s.trim().parse::<usize>())
            .collect::<Result<_, _>>()?;

        buttons.push(indices);
        pos = end + 1;
    }

    if buttons.is_empty() {
        return Err("No buttons found".into());
    }

    Ok(Machine { target, buttons })
}

/// Solve using Gaussian elimination over GF(2)
/// Returns Some(min_presses) if solvable, None otherwise
pub fn solve_machine(machine: &Machine) -> Option<usize> {
    let n_lights = machine.target.len();
    let n_buttons = machine.buttons.len();

    // Build augmented matrix [A|b] where A is button effects, b is target
    let mut matrix = vec![vec![false; n_buttons + 1]; n_lights];

    // Fill A: matrix[light][button] = true if button toggles light
    for (button_idx, button) in machine.buttons.iter().enumerate() {
        for &light_idx in button {
            if light_idx < n_lights {
                matrix[light_idx][button_idx] = true;
            }
        }
    }

    // Fill b: target state
    for (light_idx, &target) in machine.target.iter().enumerate() {
        matrix[light_idx][n_buttons] = target;
    }

    // Gaussian elimination over GF(2)
    let solution = gauss_eliminate_gf2(&mut matrix, n_buttons)?;

    // Count button presses (number of 1s in solution)
    Some(solution.iter().filter(|&&x| x).count())
}

/// Gaussian elimination over GF(2), returns solution vector if exists
fn gauss_eliminate_gf2(matrix: &mut [Vec<bool>], n_vars: usize) -> Option<Vec<bool>> {
    let n_rows = matrix.len();
    let mut pivot_col = 0;

    for row in 0..n_rows {
        if pivot_col >= n_vars {
            break;
        }

        // Find pivot
        let mut pivot_row = row;
        while pivot_row < n_rows && !matrix[pivot_row][pivot_col] {
            pivot_row += 1;
        }

        if pivot_row == n_rows {
            pivot_col += 1;
            continue;
        }

        // Swap rows
        matrix.swap(row, pivot_row);

        // Eliminate
        for other_row in 0..n_rows {
            if other_row != row && matrix[other_row][pivot_col] {
                for col in 0..=n_vars {
                    matrix[other_row][col] ^= matrix[row][col];
                }
            }
        }

        pivot_col += 1;
    }

    // Check for inconsistencies and extract solution
    let mut solution = vec![false; n_vars];

    for row in matrix.iter() {
        let has_var = row[..n_vars].iter().any(|&x| x);
        let target = row[n_vars];

        if !has_var && target {
            return None; // Inconsistent system
        }

        if has_var {
            // Find leading variable
            if let Some(col) = row[..n_vars].iter().position(|&x| x) {
                solution[col] = target;
            }
        }
    }

    Some(solution)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Machine;

    #[test]
    fn test_parse_simple() {
        let input = "[.##.] (3) (1,3) (2) {3,5,4,7}";
        let machine = parse_machine(input).unwrap();

        assert_eq!(machine.target, vec![false, true, true, false]);
        assert_eq!(machine.buttons.len(), 3);
        assert_eq!(machine.buttons[0], vec![3]);
        assert_eq!(machine.buttons[1], vec![1, 3]);
        assert_eq!(machine.buttons[2], vec![2]);
    }

    #[test]
    fn test_simple_machine() {
        let machine = Machine {
            target: vec![false, true, true, false],
            buttons: vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1],
            ],
        };

        assert_eq!(solve_machine(&machine), Some(2));
    }
}
