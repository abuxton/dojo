mod parser;
mod part1;
mod part2;

pub use parser::Machine;

pub fn solve_part1(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let machines = parser::parse_input(input)?;
    let total_machines = machines.len();
    let mut total = 0;

    eprintln!("Part 1: Processing {} machines...", total_machines);

    for (idx, machine) in machines.iter().enumerate() {
        match part1::solve_machine(machine) {
            Some(presses) => {
                total += presses;
                eprintln!(
                    "  Machine {}/{}: {} presses (running total: {})",
                    idx + 1,
                    total_machines,
                    presses,
                    total
                );
            }
            None => {
                eprintln!("  Machine {}/{}: No solution found!", idx + 1, total_machines);
                return Err("No solution found for a machine".into());
            }
        }
    }

    eprintln!("Part 1 complete: {} total presses", total);
    Ok(total)
}

pub fn solve_part2(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let machines = parser::parse_input(input)?;
    let total_machines = machines.len();
    let mut total = 0;

    eprintln!("Part 2: Processing {} machines...", total_machines);

    for (idx, machine) in machines.iter().enumerate() {
        match part2::solve_machine(machine) {
            Some(presses) => {
                total += presses;
                eprintln!(
                    "  Machine {}/{}: {} presses (running total: {})",
                    idx + 1,
                    total_machines,
                    presses,
                    total
                );
            }
            None => {
                eprintln!("  Machine {}/{}: No solution found!", idx + 1, total_machines);
                return Err("No solution found for a machine".into());
            }
        }
    }

    eprintln!("Part 2 complete: {} total presses", total);
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE).unwrap(), 7);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE).unwrap(), 33);
    }
}
