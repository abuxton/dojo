mod parser;
mod solver;

pub use parser::Machine;
pub use solver::solve_machine;

/// Solve Part 1: sum of minimum button presses for all machines
pub fn solve_part1(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let machines = parser::parse_input(input)?;
    let mut total = 0;

    for machine in machines {
        match solver::solve_machine(&machine) {
            Some(presses) => total += presses,
            None => return Err("No solution found for a machine".into()),
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_example() {
        assert_eq!(solve_part1(EXAMPLE).unwrap(), 7);
    }
}
