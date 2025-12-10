// src/part2.rs
use crate::parser::Machine;

pub fn solve_machine(machine: &Machine) -> Option<usize> {
    let n_counters = machine.joltage.len();
    let n_buttons = machine.buttons.len();

    // Build the constraint matrix
    let mut a = vec![vec![0i64; n_buttons]; n_counters];
    for (button_idx, button) in machine.buttons.iter().enumerate() {
        for &counter_idx in button {
            if counter_idx < n_counters {
                a[counter_idx][button_idx] = 1;
            }
        }
    }

    let b: Vec<i64> = machine.joltage.iter().map(|&x| x as i64).collect();

    // Use exhaustive search with better bounds
    if n_buttons <= 10 {
        if let Some(result) = solve_exhaustive(&a, &b) {
            return Some(result);
        }
    }

    None
}

fn solve_exhaustive(a: &[Vec<i64>], b: &[i64]) -> Option<usize> {
    let n_vars = a[0].len();
    let n_eqs = a.len();

    // Calculate tighter bounds for each variable
    let max_per_var: Vec<i64> = (0..n_vars)
        .map(|var_idx| {
            let mut max_val = b.iter().sum::<i64>();

            // For each equation this variable affects, it can't exceed the target
            for eq_idx in 0..n_eqs {
                if a[eq_idx][var_idx] > 0 {
                    max_val = max_val.min(b[eq_idx]);
                }
            }

            max_val
        })
        .collect();

    let mut best: Option<i64> = None;

    fn search(
        a: &[Vec<i64>],
        b: &[i64],
        current: &mut Vec<i64>,
        max_per_var: &[i64],
        idx: usize,
        current_sum: i64,
        best: &mut Option<i64>,
    ) {
        let n_vars = current.len();
        let n_eqs = a.len();

        // Prune if worse than best
        if let Some(best_val) = best {
            if current_sum >= *best_val {
                return;
            }
        }

        if idx == n_vars {
            // Check if valid
            for eq_idx in 0..n_eqs {
                let sum: i64 = (0..n_vars).map(|j| a[eq_idx][j] * current[j]).sum();
                if sum != b[eq_idx] {
                    return;
                }
            }
            // Valid and better
            *best = Some(current_sum);
            return;
        }

        // Calculate tighter bounds for this variable based on current state
        let mut min_val = 0i64;
        let mut max_val = max_per_var[idx];

        for eq_idx in 0..n_eqs {
            if a[eq_idx][idx] > 0 {
                // Calculate what's already contributed
                let sum_so_far: i64 = (0..idx).map(|j| a[eq_idx][j] * current[j]).sum();
                let remaining = b[eq_idx] - sum_so_far;

                // Can't exceed what's needed
                max_val = max_val.min(remaining);

                // Calculate what remaining variables can contribute
                let sum_remaining_vars: i64 = ((idx + 1)..n_vars)
                    .map(|j| {
                        if a[eq_idx][j] > 0 {
                            max_per_var[j]
                        } else {
                            0
                        }
                    })
                    .sum();

                // Must contribute if remaining vars can't satisfy alone
                if remaining > sum_remaining_vars {
                    min_val = min_val.max(remaining - sum_remaining_vars);
                }
            }
        }

        // Infeasible
        if min_val > max_val {
            return;
        }

        // Try values from min to max
        for val in min_val..=max_val {
            current[idx] = val;

            // Check feasibility so far
            let mut feasible = true;
            for eq_idx in 0..n_eqs {
                let sum: i64 = (0..=idx).map(|j| a[eq_idx][j] * current[j]).sum();
                if sum > b[eq_idx] {
                    feasible = false;
                    break;
                }
            }

            if feasible {
                search(a, b, current, max_per_var, idx + 1, current_sum + val, best);
            }
        }

        current[idx] = 0;
    }

    let mut current = vec![0i64; n_vars];
    search(a, b, &mut current, &max_per_var, 0, 0, &mut best);
    best.map(|x| x as usize)
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
