use crate::parser::Machine;

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

    // Gaussian elimination over GF(2) with free variable tracking
    gauss_eliminate_and_minimize(&mut matrix, n_buttons)
}

/// Gaussian elimination over GF(2) with nullspace exploration to minimize button presses
fn gauss_eliminate_and_minimize(matrix: &mut [Vec<bool>], n_vars: usize) -> Option<usize> {
    let n_rows = matrix.len();
    let mut pivot_cols = Vec::new();
    let mut row_idx = 0;

    // Forward elimination
    for col in 0..n_vars {
        // Find pivot
        let mut pivot_row = row_idx;
        while pivot_row < n_rows && !matrix[pivot_row][col] {
            pivot_row += 1;
        }

        if pivot_row == n_rows {
            continue; // This column is free
        }

        // Swap rows
        matrix.swap(row_idx, pivot_row);
        pivot_cols.push(col);

        // Eliminate
        for other_row in 0..n_rows {
            if other_row != row_idx && matrix[other_row][col] {
                for c in 0..=n_vars {
                    matrix[other_row][c] ^= matrix[row_idx][c];
                }
            }
        }

        row_idx += 1;
    }

    // Check for inconsistencies
    for row in matrix.iter() {
        let has_var = row[..n_vars].iter().any(|&x| x);
        let target = row[n_vars];
        if !has_var && target {
            return None; // Inconsistent system
        }
    }

    // Find free variables
    let mut free_vars = Vec::new();
    for col in 0..n_vars {
        if !pivot_cols.contains(&col) {
            free_vars.push(col);
        }
    }

    // If no free variables, return particular solution
    if free_vars.is_empty() {
        let mut solution = vec![false; n_vars];
        for (&col, row_idx) in pivot_cols.iter().zip(0..) {
            solution[col] = matrix[row_idx][n_vars];
        }
        return Some(solution.iter().filter(|&&x| x).count());
    }

    // Explore all combinations of free variables to find minimum
    let num_free = free_vars.len();
    let mut min_presses = usize::MAX;

    for mask in 0..(1 << num_free) {
        let mut solution = vec![false; n_vars];

        // Set free variables according to mask
        for (i, &free_col) in free_vars.iter().enumerate() {
            solution[free_col] = (mask & (1 << i)) != 0;
        }

        // Compute pivot variables
        for (row_idx, &pivot_col) in pivot_cols.iter().enumerate() {
            let mut val = matrix[row_idx][n_vars];
            for col in 0..n_vars {
                if col != pivot_col && matrix[row_idx][col] && solution[col] {
                    val ^= true;
                }
            }
            solution[pivot_col] = val;
        }

        let presses = solution.iter().filter(|&&x| x).count();
        min_presses = min_presses.min(presses);
    }

    Some(min_presses)
}

#[cfg(test)]
mod tests {
    use super::*;

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
