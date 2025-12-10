// src/part2.rs
use crate::parser::Machine;
use std::time::{Duration, Instant};

const SOLVER_TIMEOUT: Duration = Duration::from_secs(5);

pub fn solve_machine(machine: &Machine) -> Option<usize> {
    let start = Instant::now();
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

    // 1. Try branch and bound for small problems
    if n_buttons <= 6 && target.iter().max().unwrap_or(&0) <= &30 {
        if let Some(result) = branch_and_bound(&a, &target, start) {
            return Some(result);
        }
    }

    // 2. Try greedy minimize
    if let Some(result) = greedy_minimize(&a, &target) {
        return Some(result);
    }

    // 3. Try iterative solve
    if let Some(result) = iterative_solve(&a, &target) {
        return Some(result);
    }

    // 4. Try greedy with backtrack
    if let Some(result) = greedy_with_backtrack(&a, &target, start) {
        return Some(result);
    }

    None
}

fn branch_and_bound(a: &[Vec<i64>], b: &[i64], start: Instant) -> Option<usize> {
    let n_vars = a[0].len();
    let max_val = *b.iter().max().unwrap_or(&0);

    let mut best_solution: Option<(Vec<i64>, i64)> = None;
    let mut current = vec![0i64; n_vars];

    fn search(
        a: &[Vec<i64>],
        b: &[i64],
        current: &mut Vec<i64>,
        idx: usize,
        best: &mut Option<(Vec<i64>, i64)>,
        max_val: i64,
        start: Instant,
    ) -> bool {
        if start.elapsed() > SOLVER_TIMEOUT {
            return false;
        }

        let n_vars = current.len();
        let n_eqs = a.len();

        if idx == n_vars {
            for i in 0..n_eqs {
                let sum: i64 = (0..n_vars).map(|j| a[i][j] * current[j]).sum();
                if sum != b[i] {
                    return true;
                }
            }

            let total: i64 = current.iter().sum();
            if best.is_none() || total < best.as_ref().unwrap().1 {
                *best = Some((current.clone(), total));
            }
            return true;
        }

        if let Some((_, best_sum)) = best {
            if current.iter().sum::<i64>() >= *best_sum {
                return true;
            }
        }

        let mut upper = max_val;
        for i in 0..n_eqs {
            if a[i][idx] > 0 {
                let current_sum: i64 = (0..idx).map(|j| a[i][j] * current[j]).sum();
                let remaining = b[i] - current_sum;
                if remaining < 0 {
                    return true;
                }
                let max_for_eq = remaining / a[i][idx];
                upper = upper.min(max_for_eq);
            }
        }

        for val in 0..=upper.min(max_val) {
            current[idx] = val;

            let mut feasible = true;
            for i in 0..n_eqs {
                let sum: i64 = (0..=idx).map(|j| a[i][j] * current[j]).sum();
                if sum > b[i] {
                    feasible = false;
                    break;
                }
            }

            if feasible && !search(a, b, current, idx + 1, best, max_val, start) {
                return false;
            }
        }

        current[idx] = 0;
        true
    }

    if search(a, b, &mut current, 0, &mut best_solution, max_val, start) {
        best_solution.map(|(_, sum)| sum as usize)
    } else {
        None
    }
}

fn greedy_minimize(a: &[Vec<i64>], b: &[i64]) -> Option<usize> {
    let n_vars = a[0].len();
    let n_eqs = a.len();
    let mut solution = vec![0i64; n_vars];

    for eq_idx in 0..n_eqs {
        loop {
            let current_sum: i64 = (0..n_vars).map(|j| a[eq_idx][j] * solution[j]).sum();
            let diff = b[eq_idx] - current_sum;

            if diff == 0 {
                break;
            }

            if diff > 0 {
                let mut found = false;
                for var_idx in 0..n_vars {
                    if a[eq_idx][var_idx] > 0 {
                        let increment = (diff / a[eq_idx][var_idx]).max(1);
                        solution[var_idx] += increment;
                        found = true;
                        break;
                    }
                }
                if !found {
                    return None;
                }
            } else {
                let mut found = false;
                for var_idx in (0..n_vars).rev() {
                    if a[eq_idx][var_idx] > 0 && solution[var_idx] > 0 {
                        let decrement = ((-diff) / a[eq_idx][var_idx]).max(1).min(solution[var_idx]);
                        solution[var_idx] -= decrement;
                        found = true;
                        break;
                    }
                }
                if !found {
                    return None;
                }
            }
        }
    }

    for _round in 0..1000 {
        let mut all_good = true;

        for eq_idx in 0..n_eqs {
            let sum: i64 = (0..n_vars).map(|j| a[eq_idx][j] * solution[j]).sum();
            let diff = b[eq_idx] - sum;

            if diff == 0 {
                continue;
            }

            all_good = false;

            if diff > 0 {
                for var_idx in 0..n_vars {
                    if a[eq_idx][var_idx] > 0 {
                        solution[var_idx] += 1;
                        break;
                    }
                }
            } else {
                for var_idx in 0..n_vars {
                    if a[eq_idx][var_idx] > 0 && solution[var_idx] > 0 {
                        solution[var_idx] -= 1;
                        break;
                    }
                }
            }
        }

        if all_good {
            break;
        }
    }

    for i in 0..n_eqs {
        let sum: i64 = (0..n_vars).map(|j| a[i][j] * solution[j]).sum();
        if sum != b[i] {
            return None;
        }
    }

    Some(solution.iter().sum::<i64>() as usize)
}

fn iterative_solve(a: &[Vec<i64>], b: &[i64]) -> Option<usize> {
    let n_vars = a[0].len();
    let n_eqs = a.len();
    let mut solution = vec![0i64; n_vars];

    for var_idx in 0..n_vars {
        let mut total_needed = 0i64;
        let mut count = 0;

        for eq_idx in 0..n_eqs {
            if a[eq_idx][var_idx] > 0 {
                total_needed += b[eq_idx];
                count += 1;
            }
        }

        if count > 0 {
            solution[var_idx] = (total_needed / (count as i64)).max(0);
        }
    }

    for _iteration in 0..10000 {
        let mut changed = false;

        for eq_idx in 0..n_eqs {
            let sum: i64 = (0..n_vars).map(|j| a[eq_idx][j] * solution[j]).sum();
            let diff = b[eq_idx] - sum;

            if diff != 0 {
                let mut best_var = None;
                let mut best_impact = 0;

                for var_idx in 0..n_vars {
                    if a[eq_idx][var_idx] > 0 && (diff > 0 || solution[var_idx] > 0) {
                        best_var = Some(var_idx);
                        best_impact = a[eq_idx][var_idx];
                        break;
                    }
                }

                if let Some(var_idx) = best_var {
                    if diff > 0 {
                        solution[var_idx] += (diff / best_impact).max(1);
                    } else if solution[var_idx] > 0 {
                        let decrement = ((-diff) / best_impact).max(1).min(solution[var_idx]);
                        solution[var_idx] -= decrement;
                    }
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }
    }

    for i in 0..n_eqs {
        let sum: i64 = (0..n_vars).map(|j| a[i][j] * solution[j]).sum();
        if sum != b[i] {
            return None;
        }
    }

    Some(solution.iter().sum::<i64>() as usize)
}

fn greedy_with_backtrack(a: &[Vec<i64>], b: &[i64], start: Instant) -> Option<usize> {
    let n_vars = a[0].len();
    let n_eqs = a.len();

    // Fixed: Create strategies with correct size (n_vars, not n_eqs)
    let strategies = vec![
        vec![0i64; n_vars],  // Start from zero
        vec![b.iter().sum::<i64>() / (n_vars as i64); n_vars],  // Equal distribution
    ];

    for initial in strategies {
        if start.elapsed() > SOLVER_TIMEOUT {
            break;
        }

        let mut solution = initial;

        for _attempt in 0..100 {
            let mut improved = false;

            for eq_idx in 0..n_eqs {
                let sum: i64 = (0..n_vars).map(|j| a[eq_idx][j] * solution[j]).sum();
                let diff = b[eq_idx] - sum;

                if diff != 0 {
                    for var_idx in 0..n_vars {
                        if a[eq_idx][var_idx] > 0 {
                            if diff > 0 {
                                solution[var_idx] += 1;
                            } else if solution[var_idx] > 0 {
                                solution[var_idx] -= 1;
                            }
                            improved = true;
                            break;
                        }
                    }
                }
            }

            if !improved {
                let mut valid = true;
                for i in 0..n_eqs {
                    let sum: i64 = (0..n_vars).map(|j| a[i][j] * solution[j]).sum();
                    if sum != b[i] {
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
