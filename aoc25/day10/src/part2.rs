use crate::parser::Machine;

/// Solve Part 2: minimize button presses to reach joltage targets
pub fn solve_machine(machine: &Machine) -> Option<usize> {
    let n_counters = machine.joltage.len();
    let n_buttons = machine.buttons.len();

    // Build coefficient matrix A where A[counter][button] = 1 if button affects counter
    let mut a = vec![vec![0i64; n_buttons]; n_counters];

    for (button_idx, button) in machine.buttons.iter().enumerate() {
        for &counter_idx in button {
            if counter_idx < n_counters {
                a[counter_idx][button_idx] = 1;
            }
        }
    }

    let target: Vec<i64> = machine.joltage.iter().map(|&x| x as i64).collect();

    // Use greedy solver for all cases
    solve_greedy(&a, &target)
}

/// Greedy solver: process equations in order, satisfy each with minimal button presses
fn solve_greedy(a: &[Vec<i64>], b: &[i64]) -> Option<usize> {
    let n_vars = a[0].len();
    let n_eqs = a.len();

    let mut solution = vec![0i64; n_vars];
    let mut remaining = b.to_vec();

    // Create equation priority: process equations with fewer non-zero coefficients first
    let mut eq_order: Vec<usize> = (0..n_eqs).collect();
    eq_order.sort_by_key(|&i| a[i].iter().filter(|&&x| x != 0).count());

    for &eq_idx in &eq_order {
        if remaining[eq_idx] == 0 {
            continue;
        }

        if remaining[eq_idx] < 0 {
            // Overshot, try to reduce
            // Find a variable that affects this equation but not others we've satisfied
            let mut reduced = false;
            for var_idx in 0..n_vars {
                if a[eq_idx][var_idx] > 0 && solution[var_idx] > 0 {
                    let reduction = solution[var_idx].min((-remaining[eq_idx]).abs());
                    solution[var_idx] -= reduction;

                    // Update all equations
                    for i in 0..n_eqs {
                        remaining[i] += a[i][var_idx] * reduction;
                    }

                    reduced = true;
                    if remaining[eq_idx] >= 0 {
                        break;
                    }
                }
            }

            if !reduced || remaining[eq_idx] < 0 {
                return None; // Can't fix overshoot
            }
        }

        // Find the variable that affects this equation most efficiently
        let mut best_var = None;
        let mut best_score = i64::MAX;

        for var_idx in 0..n_vars {
            if a[eq_idx][var_idx] > 0 {
                // Score = number of other unsatisfied equations this affects
                let other_effects: i64 = (0..n_eqs)
                    .filter(|&i| i != eq_idx && remaining[i] > 0 && a[i][var_idx] > 0)
                    .count() as i64;

                // Prefer variables that also help other equations
                if best_var.is_none() || other_effects > best_score {
                    best_var = Some(var_idx);
                    best_score = other_effects;
                }
            }
        }

        if let Some(var_idx) = best_var {
            let needed = remaining[eq_idx] / a[eq_idx][var_idx];
            let remainder = remaining[eq_idx] % a[eq_idx][var_idx];

            if remainder != 0 {
                // Can't satisfy exactly with this variable, try combination
                // For now, just use ceiling division
                let needed = (remaining[eq_idx] + a[eq_idx][var_idx] - 1) / a[eq_idx][var_idx];
                solution[var_idx] += needed;

                // Update all equations
                for i in 0..n_eqs {
                    remaining[i] -= a[i][var_idx] * needed;
                }
            } else {
                solution[var_idx] += needed;

                // Update all equations
                for i in 0..n_eqs {
                    remaining[i] -= a[i][var_idx] * needed;
                }
            }
        } else {
            return None; // No variable affects this equation
        }
    }

    // Verify solution
    for i in 0..n_eqs {
        let sum: i64 = (0..n_vars).map(|j| a[i][j] * solution[j]).sum();
        if sum != b[i] {
            // Try to fix small discrepancies
            let diff = b[i] - sum;
            if diff != 0 {
                // Find any variable that affects this equation
                let mut fixed = false;
                for var_idx in 0..n_vars {
                    if a[i][var_idx] > 0 {
                        let adjustment = diff / a[i][var_idx];
                        if diff % a[i][var_idx] == 0 && solution[var_idx] + adjustment >= 0 {
                            solution[var_idx] += adjustment;
                            fixed = true;
                            break;
                        }
                    }
                }

                if !fixed {
                    return None;
                }
            }
        }
    }

    // Final verification
    for i in 0..n_eqs {
        let sum: i64 = (0..n_vars).map(|j| a[i][j] * solution[j]).sum();
        if sum != b[i] {
            return None;
        }
    }

    Some(solution.iter().sum::<i64>() as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joltage_machine() {
        let machine = Machine {
            target: vec![],
            buttons: vec![vec![3], vec![1, 3], vec![2], vec![2, 3], vec![0, 2], vec![0, 1]],
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

        // Should find a solution
        assert!(solve_machine(&machine).is_some());
    }
}
