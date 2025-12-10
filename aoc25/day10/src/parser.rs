use std::error::Error;

#[derive(Debug, Clone)]
pub struct Machine {
    pub target: Vec<bool>,
    pub buttons: Vec<Vec<usize>>,
}

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
