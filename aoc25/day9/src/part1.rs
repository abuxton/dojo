use crate::shared::{parse, Point};

/// Find the largest rectangle with opposite corners both being red tiles.
pub fn solve_part1(input: &str) -> u32 {
    let points = parse(input);

    if points.len() < 2 {
        return 0;
    }

    let mut max_area: u32 = 0;
    let points_vec: Vec<Point> = points.iter().copied().collect();

    // Try all pairs of points as opposite corners
    for i in 0..points_vec.len() {
        for j in i + 1..points_vec.len() {
            let p1 = points_vec[i];
            let p2 = points_vec[j];

            // Calculate rectangle bounds
            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);

            // Calculate area (both p1 and p2 are guaranteed to be red tiles)
            let width = (max_x - min_x) as u32;
            let height = (max_y - min_y) as u32;
            let area = width * height;

            if area > max_area {
                eprintln!(
                    "New max: ({},{}) to ({},{}): {}×{} = {}",
                    min_x, min_y, max_x, max_y, width, height, area
                );
                max_area = area;
            }
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn example_largest_rectangle() {
        assert_eq!(solve_part1(EXAMPLE), 50);
    }
}
