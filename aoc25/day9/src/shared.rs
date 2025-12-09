use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Parse comma-separated x,y coordinates into a HashSet of points.
pub fn parse(input: &str) -> HashSet<Point> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let parts: Vec<&str> = l.split(',').collect();
            let x = parts[0].trim().parse::<i32>().expect("bad x coord");
            let y = parts[1].trim().parse::<i32>().expect("bad y coord");
            Point { x, y }
        })
        .collect()
}
