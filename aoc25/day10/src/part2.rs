// src/part2.rs
use crate::parser::Machine;
use std::collections::HashSet;
use std::time::{Duration, Instant};

const SOLVER_TIMEOUT: Duration = Duration::from_secs(10);

pub fn solve_machine(machine: &Machine) -> Option<usize> {
    let start_time = Instant::now();
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

    // Strategy 1: For very small problems, use exhaustive search
    if n_buttons <= 8 && target.iter().sum::<i64>() <= 100 {
        if let Some(result) = solve_branch_and_bound(&a, &target, start_time) {
            return Some(result);
        }
    }

    // Strategy 2: Iterative repair with state tracking
    if start_time.elapsed() < SOLVER_TIMEOUT {
        if let Some(result) = solve_iterative_repair(&a, &target, start_time) {
            return Some(result);
        }
    }

    // Strategy 3: Multiple random starts
    for seed in 0..5 {
        if start_time.elapsed() >= SOLVER_TIMEOUT {
            break;
        }
        if let Some(result) = solve_random_start(&a, &target, seed, start_time) {
            return Some(result);
        }
    }

    None
}

fn solve_branch_and_bound(a: &[Vec<i64>], b: &[i64], start_time: Instant) -> Option<usize> {
    if start_time.elapsed() >= SOLVER_TIMEOUT {
        return None;
    }

    let n_vars = a[0].len();
    let n_eqs = a.len();

    let max_per_var: Vec<i64> = (0..n_vars)
        .map(|var_idx| {
            let mut max_val = b.iter().sum::<i64>();
            for eq_idx in 0..n_eqs {
                if a[eq_idx][var_idx] > 0 {
                    max_val = max_val.min(b[eq_idx]);
                }
            }
            max_val
        })
        .collect();

    let mut best: Option<i64> = None;
    let mut current = vec![0i64; n_vars];

    fn search(
        a: &[Vec<i64>],
        b: &[i64],
        current: &mut Vec<i64>,
        max_per_var: &[i64],
        idx: usize,
        current_sum: i64,
        best: &mut Option<i64>,
        start_time: Instant,
    ) -> bool {
        if start_time.elapsed() >= SOLVER_TIMEOUT {
            return false;
        }

        let n_vars = current.len();
        let n_eqs = a.len();

        if let Some(best_val) = best {
            if current_sum >= *best_val {
                return true;
            }
        }

        if idx == n_vars {
            for eq_idx in 0..n_eqs {
                let sum: i64 = (0..n_vars).map(|j| a[eq_idx][j] * current[j]).sum();
                if sum != b[eq_idx] {
                    return true;
                }
            }
            *best = Some(current_sum);
            return true;
        }

        let mut min_val = 0i64;
        let mut max_val = max_per_var[idx];

        for eq_idx in 0..n_eqs {
            if a[eq_idx][idx] > 0 {
                let sum_so_far: i64 = (0..idx).map(|j| a[eq_idx][j] * current[j]).sum();
                let remaining = b[eq_idx] - sum_so_far;
                max_val = max_val.min(remaining);

                let sum_remaining_vars: i64 = ((idx + 1)..n_vars)
                    .map(|j| if a[eq_idx][j] > 0 { max_per_var[j] } else { 0 })
                    .sum();

                if remaining > sum_remaining_vars {
                    min_val = min_val.max(remaining - sum_remaining_vars);
                }
            }
        }

        if min_val > max_val {
            return true;
        }

        for val in min_val..=max_val {
            current[idx] = val;

            let mut feasible = true;
            for eq_idx in 0..n_eqs {
                let sum: i64 = (0..=idx).map(|j| a[eq_idx][j] * current[j]).sum();
                if sum > b[eq_idx] {
                    feasible = false;
                    break;
                }
            }

            if feasible
                && !search(
                    a,
                    b,
                    current,
                    max_per_var,
                    idx + 1,
                    current_sum + val,
                    best,
                    start_time,
                )
            {
                return false;
            }
        }

        current[idx] = 0;
        true
    }

    if search(a, b, &mut current, &max_per_var, 0, 0, &mut best, start_time) {
        best.map(|x| x as usize)
    } else {
        None
    }
}

/// Iterative repair with cycle detection
fn solve_iterative_repair(a: &[Vec<i64>], b: &[i64], start_time: Instant) -> Option<usize> {
    let n_vars = a[0].len();
    let n_eqs = a.len();
    let mut solution = vec![0i64; n_vars];

    // Initialize with upper bound estimate
    for eq_idx in 0..n_eqs {
        let target = b[eq_idx];
        let affecting: Vec<usize> = (0..n_vars).filter(|&i| a[eq_idx][i] > 0).collect();

        if !affecting.is_empty() {
            for &var_idx in &affecting {
                solution[var_idx] = solution[var_idx].max(target);
            }
        }
    }

    // Track visited states to detect cycles
    let mut seen_states: HashSet<Vec<i64>> = HashSet::new();

    for _ in 0..1000 {
        if start_time.elapsed() >= SOLVER_TIMEOUT {
            return None;
        }

        // Check if we've seen this state before (cycle detection)
        if !seen_states.insert(solution.clone()) {
            return None; // Cycle detected
        }

        let mut all_satisfied = true;

        for eq_idx in 0..n_eqs {
            let sum: i64 = (0..n_vars).map(|j| a[eq_idx][j] * solution[j]).sum();
            let diff = b[eq_idx] - sum;

            if diff != 0 {
                all_satisfied = false;

                // Find buttons affecting this equation, sorted by total coverage
                let mut buttons_with_coverage: Vec<(usize, usize)> = (0..n_vars)
                    .filter(|&i| a[eq_idx][i] > 0)
                    .map(|i| {
                        let coverage = (0..n_eqs).filter(|&e| a[e][i] > 0).count();
                        (i, coverage)
                    })
                    .collect();

                // Sort by coverage (prefer buttons that affect fewer equations)
                buttons_with_coverage.sort_by_key(|x| x.1);

                if let Some(&(var_idx, _)) = buttons_with_coverage.first() {
                    if diff > 0 {
                        solution[var_idx] += diff;
                    } else if solution[var_idx] >= diff.abs() {
                        solution[var_idx] -= diff.abs();
                    } else if solution[var_idx] > 0 {
                        // Can't fix entirely with this button, reduce what we can
                        solution[var_idx] = 0;
                    }
                }
            }
        }

        if all_satisfied {
            // Try to optimize
            for var_idx in 0..n_vars {
                while solution[var_idx] > 0 {
                    solution[var_idx] -= 1;

                    let mut valid = true;
                    for eq_idx in 0..n_eqs {
                        let sum: i64 = (0..n_vars).map(|j| a[eq_idx][j] * solution[j]).sum();
                        if sum != b[eq_idx] {
                            valid = false;
                            break;
                        }
                    }

                    if !valid {
                        solution[var_idx] += 1;
                        break;
                    }
                }
            }

            return Some(solution.iter().sum::<i64>() as usize);
        }
    }

    None
}

/// Try different random starting points
fn solve_random_start(
    a: &[Vec<i64>],
    b: &[i64],
    seed: usize,
    start_time: Instant,
) -> Option<usize> {
    let n_vars = a[0].len();
    let n_eqs = a.len();
    let mut solution = vec![0i64; n_vars];

    // Different initialization strategies based on seed
    match seed {
        0 => {
            // Start from zero
        }
        1 => {
            // Start with equal distribution
            let total: i64 = b.iter().sum();
            let per_var = total / n_vars as i64;
            solution = vec![per_var; n_vars];
        }
        2 => {
            // Start with equation targets
            for eq_idx in 0..n_eqs {
                let affecting: Vec<usize> =
                    (0..n_vars).filter(|&i| a[eq_idx][i] > 0).collect();
                if !affecting.is_empty() {
                    let per_button = b[eq_idx] / affecting.len() as i64;
                    for &var_idx in &affecting {
                        solution[var_idx] = solution[var_idx].max(per_button);
                    }
                }
            }
        }
        3 => {
            // Start with dedicated buttons
            for eq_idx in 0..n_eqs {
                for var_idx in 0..n_vars {
                    if a[eq_idx][var_idx] > 0 {
                        let other_eqs = (0..n_eqs)
                            .filter(|&e| e != eq_idx && a[e][var_idx] > 0)
                            .count();
                        if other_eqs == 0 {
                            solution[var_idx] = b[eq_idx];
                            break;
                        }
                    }
                }
            }
        }
        _ => {
            // Reverse order processing
            for eq_idx in (0..n_eqs).rev() {
                let affecting: Vec<usize> =
                    (0..n_vars).filter(|&i| a[eq_idx][i] > 0).collect();
                if !affecting.is_empty() {
                    solution[affecting[0]] = b[eq_idx];
                }
            }
        }
    }

    // Now repair
    for _ in 0..500 {
        if start_time.elapsed() >= SOLVER_TIMEOUT {
            return None;
        }

        let mut changed = false;

        for eq_idx in 0..n_eqs {
            let sum: i64 = (0..n_vars).map(|j| a[eq_idx][j] * solution[j]).sum();
            let diff = b[eq_idx] - sum;

            if diff != 0 {
                changed = true;

                for var_idx in 0..n_vars {
                    if a[eq_idx][var_idx] > 0 {
                        if diff > 0 {
                            solution[var_idx] += diff;
                        } else if solution[var_idx] >= diff.abs() {
                            solution[var_idx] -= diff.abs();
                        }
                        break;
                    }
                }
            }
        }

        if !changed {
            // Verify
            let mut valid = true;
            for eq_idx in 0..n_eqs {
                let sum: i64 = (0..n_vars).map(|j| a[eq_idx][j] * solution[j]).sum();
                if sum != b[eq_idx] {
                    valid = false;
                    break;
                }
            }

            if valid {
                return Some(solution.iter().sum::<i64>() as usize);
            }
            break;
        }
    }

    None
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
