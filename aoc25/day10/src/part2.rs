// src/part2.rs
use crate::parser::Machine;
use std::time::{Duration, Instant};

const EXHAUSTIVE_TIMEOUT: Duration = Duration::from_secs(2);

pub fn solve_machine(machine: &Machine) -> Option<usize> {
    let n_counters = machine.joltage.len();
    let n_buttons = machine.buttons.len();

    let mut a = vec![vec![0i64; n_buttons]; n_counters];

    for (button_idx, button) in machine.buttons.iter().enumerate() {
        for &counter_idx in button {
            if counter_idx < n_counters {
                a[counter_idx][button_idx] = 1;
            }
        }
    }

    let target: Vec<i64> = machine.joltage.iter().map(|&x| x as i64).collect();

    // Try strategies and pick the best (minimal) solution
    let mut best_solution: Option<usize> = None;

    if let Some(result) = solve_direct(&a, &target) {
        best_solution = Some(result);
    }

    if let Some(result) = solve_lp_relaxation(&a, &target) {
        best_solution = match best_solution {
            Some(prev) => Some(prev.min(result)),
            None => Some(result),
        };
    }

    if let Some(result) = solve_gaussian(&a, &target) {
        best_solution = match best_solution {
            Some(prev) => Some(prev.min(result)),
            None => Some(result),
        };
    }

    // Only try exhaustive for very small problems (≤6 buttons, small targets)
    if best_solution.is_none()
        && n_buttons <= 6
        && target.iter().max().unwrap_or(&0) <= &50
    {
        let start = Instant::now();
        if let Some(result) = solve_exhaustive(&a, &target, 100, start) {
            best_solution = Some(result);
        }
    }

    best_solution
}

/// Exhaustive search with pruning and timeout for very small problems
fn solve_exhaustive(a: &[Vec<i64>], b: &[i64], max_total: i64, start: Instant) -> Option<usize> {
    let n_vars = a[0].len();
    let mut best: Option<i64> = None;
    let mut solution = vec![0i64; n_vars];

    fn search(
        a: &[Vec<i64>],
        b: &[i64],
        solution: &mut Vec<i64>,
        idx: usize,
        current_total: i64,
        best: &mut Option<i64>,
        max_total: i64,
        start: Instant,
    ) -> bool {
        // Timeout check
        if start.elapsed() > EXHAUSTIVE_TIMEOUT {
            return false;
        }

        let n_vars = solution.len();
        let n_eqs = a.len();

        // Prune if we've exceeded best or max
        if let Some(best_val) = best {
            if current_total >= *best_val {
                return true;
            }
        }
        if current_total > max_total {
            return true;
        }

        if idx == n_vars {
            // Check if valid
            for i in 0..n_eqs {
                let sum: i64 = (0..n_vars).map(|j| a[i][j] * solution[j]).sum();
                if sum != b[i] {
                    return true;
                }
            }
            // Valid solution found
            *best = Some(current_total);
            return true;
        }

        // Calculate upper bound for this variable
        let mut max_val = max_total - current_total;
        for i in 0..n_eqs {
            if a[i][idx] > 0 {
                let sum_so_far: i64 = (0..idx).map(|j| a[i][j] * solution[j]).sum();
                let remaining = b[i] - sum_so_far;
                if remaining < 0 {
                    return true;
                }
                if remaining >= 0 {
                    max_val = max_val.min(remaining);
                }
            }
        }

        // Try values (limit iterations)
        for val in 0..=max_val.min(50) {
            solution[idx] = val;

            // Early feasibility check
            let mut feasible = true;
            for i in 0..n_eqs {
                let sum: i64 = (0..=idx).map(|j| a[i][j] * solution[j]).sum();
                if sum > b[i] {
                    feasible = false;
                    break;
                }
            }

            if !feasible {
                solution[idx] = 0;
                continue;
            }

            if !search(a, b, solution, idx + 1, current_total + val, best, max_total, start) {
                return false; // Timeout
            }
        }
        solution[idx] = 0;
        true
    }

    if search(a, b, &mut solution, 0, 0, &mut best, max_total, start) {
        best.map(|x| x as usize)
    } else {
        None // Timeout
    }
}

fn solve_direct(a: &[Vec<i64>], b: &[i64]) -> Option<usize> {
    let n_vars = a[0].len();
    let n_eqs = a.len();
    let mut solution = vec![0i64; n_vars];

    // For each counter, find buttons that affect ONLY that counter
    for eq_idx in 0..n_eqs {
        let target_val = b[eq_idx];
        let mut buttons_for_this: Vec<usize> = Vec::new();

        for var_idx in 0..n_vars {
            if a[eq_idx][var_idx] > 0 {
                let mut affects_others = false;
                for other_eq in 0..n_eqs {
                    if other_eq != eq_idx && a[other_eq][var_idx] > 0 {
                        affects_others = true;
                        break;
                    }
                }
                if !affects_others {
                    buttons_for_this.push(var_idx);
                }
            }
        }

        if !buttons_for_this.is_empty() {
            let button = buttons_for_this[0];
            solution[button] = target_val;
        }
    }

    // Verify and adjust
    for _ in 0..1000 {
        let mut all_satisfied = true;

        for eq_idx in 0..n_eqs {
            let sum: i64 = (0..n_vars).map(|j| a[eq_idx][j] * solution[j]).sum();
            let diff = b[eq_idx] - sum;

            if diff != 0 {
                all_satisfied = false;

                for var_idx in 0..n_vars {
                    if a[eq_idx][var_idx] > 0 {
                        if diff > 0 {
                            solution[var_idx] += diff;
                        } else if solution[var_idx] >= -diff {
                            solution[var_idx] += diff;
                        }
                        break;
                    }
                }
            }
        }

        if all_satisfied {
            return Some(solution.iter().sum::<i64>() as usize);
        }
    }

    None
}

fn solve_lp_relaxation(a: &[Vec<i64>], b: &[i64]) -> Option<usize> {
    let n_vars = a[0].len();
    let n_eqs = a.len();

    let af: Vec<Vec<f64>> = a
        .iter()
        .map(|row| row.iter().map(|&x| x as f64).collect())
        .collect();
    let bf: Vec<f64> = b.iter().map(|&x| x as f64).collect();

    let mut solution_f = vec![0.0; n_vars];

    let mut var_utility: Vec<(usize, usize)> = Vec::new();
    for var_idx in 0..n_vars {
        let mut count = 0;
        for eq_idx in 0..n_eqs {
            if af[eq_idx][var_idx] > 0.0 {
                count += 1;
            }
        }
        var_utility.push((var_idx, count));
    }

    var_utility.sort_by(|a, b| b.1.cmp(&a.1));

    for eq_idx in 0..n_eqs {
        let current_sum: f64 = (0..n_vars).map(|j| af[eq_idx][j] * solution_f[j]).sum();
        let diff = bf[eq_idx] - current_sum;

        if diff > 0.0 {
            for &(var_idx, _) in &var_utility {
                if af[eq_idx][var_idx] > 0.0 {
                    solution_f[var_idx] += diff / af[eq_idx][var_idx];
                    break;
                }
            }
        }
    }

    let mut solution: Vec<i64> = solution_f.iter().map(|&x| x.ceil() as i64).collect();

    // Refine with optimization: try to reduce each variable
    for _ in 0..1000 {
        let mut improved = false;

        // Try reducing each variable
        for var_idx in 0..n_vars {
            if solution[var_idx] > 0 {
                solution[var_idx] -= 1;

                // Check if still valid
                let mut valid = true;
                for eq_idx in 0..n_eqs {
                    let sum: i64 = (0..n_vars).map(|j| a[eq_idx][j] * solution[j]).sum();
                    if sum < b[eq_idx] {
                        valid = false;
                        break;
                    }
                }

                if !valid {
                    solution[var_idx] += 1; // Restore
                }
            }
        }

        // Fix any deficits
        for eq_idx in 0..n_eqs {
            let sum: i64 = (0..n_vars).map(|j| a[eq_idx][j] * solution[j]).sum();
            let diff = b[eq_idx] - sum;

            if diff != 0 {
                improved = true;
                for var_idx in 0..n_vars {
                    if a[eq_idx][var_idx] > 0 {
                        if diff > 0 {
                            solution[var_idx] += 1;
                        } else if solution[var_idx] > 0 {
                            solution[var_idx] -= 1;
                        }
                        break;
                    }
                }
            }
        }

        if !improved {
            break;
        }
    }

    // Final verification
    for eq_idx in 0..n_eqs {
        let sum: i64 = (0..n_vars).map(|j| a[eq_idx][j] * solution[j]).sum();
        if sum != b[eq_idx] {
            return None;
        }
    }

    Some(solution.iter().sum::<i64>() as usize)
}

fn solve_gaussian(a: &[Vec<i64>], b: &[i64]) -> Option<usize> {
    let n_vars = a[0].len();
    let n_eqs = a.len();

    let mut aug: Vec<Vec<f64>> = Vec::new();
    for i in 0..n_eqs {
        let mut row = Vec::new();
        for j in 0..n_vars {
            row.push(a[i][j] as f64);
        }
        row.push(b[i] as f64);
        aug.push(row);
    }

    for col in 0..n_vars.min(n_eqs) {
        let mut max_row = col;
        for row in col..n_eqs {
            if aug[row][col].abs() > aug[max_row][col].abs() {
                max_row = row;
            }
        }

        if aug[max_row][col].abs() < 1e-10 {
            continue;
        }

        aug.swap(col, max_row);

        for row in col + 1..n_eqs {
            let factor = aug[row][col] / aug[col][col];
            for c in col..=n_vars {
                aug[row][c] -= factor * aug[col][c];
            }
        }
    }

    let mut solution = vec![0.0; n_vars];
    for i in (0..n_eqs.min(n_vars)).rev() {
        let mut sum = aug[i][n_vars];
        for j in i + 1..n_vars {
            sum -= aug[i][j] * solution[j];
        }
        if aug[i][i].abs() > 1e-10 {
            solution[i] = (sum / aug[i][i]).max(0.0);
        }
    }

    let mut int_solution: Vec<i64> = solution.iter().map(|&x| x.round() as i64).collect();

    // Refine with optimization
    for _ in 0..1000 {
        let mut improved = false;

        // Try reducing variables
        for var_idx in 0..n_vars {
            if int_solution[var_idx] > 0 {
                int_solution[var_idx] -= 1;

                let mut valid = true;
                for eq_idx in 0..n_eqs {
                    let sum: i64 = (0..n_vars).map(|j| a[eq_idx][j] * int_solution[j]).sum();
                    if sum < b[eq_idx] {
                        valid = false;
                        break;
                    }
                }

                if !valid {
                    int_solution[var_idx] += 1;
                }
            }
        }

        // Fix deficits
        for eq_idx in 0..n_eqs {
            let sum: i64 = (0..n_vars).map(|j| a[eq_idx][j] * int_solution[j]).sum();
            let diff = b[eq_idx] - sum;

            if diff != 0 {
                improved = true;
                for var_idx in 0..n_vars {
                    if a[eq_idx][var_idx] > 0 {
                        if diff > 0 {
                            int_solution[var_idx] += 1;
                        } else if int_solution[var_idx] > 0 {
                            int_solution[var_idx] -= 1;
                        }
                        break;
                    }
                }
            }
        }

        if !improved {
            break;
        }
    }

    // Final verification
    for eq_idx in 0..n_eqs {
        let sum: i64 = (0..n_vars).map(|j| a[eq_idx][j] * int_solution[j]).sum();
        if sum != b[eq_idx] {
            return None;
        }
    }

    Some(int_solution.iter().sum::<i64>() as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joltage_machine() {
        let machine = Machine {
            target: vec![],
            buttons: vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1],
            ],
            joltage: vec![3, 5, 4, 7],
        };

        assert_eq!(solve_machine(&machine), Some(10));
    }

    #[test]
    fn test_simple_case() {
        let machine = Machine {
            target: vec![],
            buttons: vec![vec![0, 2], vec![1, 2, 3], vec![2, 3]],
            joltage: vec![110, 10, 120, 10],
        };

        assert!(solve_machine(&machine).is_some());
    }

    #[test]
    fn test_example_machine_2() {
        let machine = Machine {
            target: vec![],
            buttons: vec![
                vec![0, 2, 3, 4],
                vec![2, 3],
                vec![0, 4],
                vec![0, 1, 2],
                vec![1, 2, 3, 4],
            ],
            joltage: vec![7, 5, 12, 7, 2],
        };

        assert_eq!(solve_machine(&machine), Some(12));
    }

    #[test]
    fn test_first_real_machine() {
        let machine = Machine {
            target: vec![],
            buttons: vec![vec![0], vec![1]],
            joltage: vec![5, 3],
        };

        let result = solve_machine(&machine);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 8);
    }
}
