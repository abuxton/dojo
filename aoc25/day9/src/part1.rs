use crate::shared::{Point, parse_points_set};
use std::collections::HashSet;

/// Find the largest rectangle with opposite corners both being red tiles.
/// Area includes the boundary (inclusive of both endpoints).
pub fn solve_part1(input: &str) -> u64 {
    let points_set = parse_points_set(input);
    if points_set.len() < 2 {
        return 0;
    }
    let points_vec: Vec<Point> = points_set.iter().copied().collect();
    let mut max_area: u64 = 0;

    for i in 0..points_vec.len() {
        for j in i + 1..points_vec.len() {
            let p1 = points_vec[i];
            let p2 = points_vec[j];

            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);

            let width = (max_x - min_x + 1) as u64;
            let height = (max_y - min_y + 1) as u64;
            let area = width * height;

            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

/// Visualize the floor with red tiles and optional rectangle overlay.
/// Automatically determines grid bounds from the points or uses fixed bounds for examples.
pub fn visualize_floor(
    points: &HashSet<Point>,
    rect: Option<(Point, Point)>,
    fixed_bounds: bool,
) -> String {
    if points.is_empty() {
        return String::new();
    }

    let (grid_min_x, grid_max_x, grid_min_y, grid_max_y) = if fixed_bounds {
        // Use fixed grid for examples (0-13 x, 0-8 y)
        (0, 13, 0, 8)
    } else {
        // Dynamic bounds based on actual points
        let min_x = points.iter().map(|p| p.x).min().unwrap();
        let max_x = points.iter().map(|p| p.x).max().unwrap();
        let min_y = points.iter().map(|p| p.y).min().unwrap();
        let max_y = points.iter().map(|p| p.y).max().unwrap();
        (min_x, max_x, min_y, max_y)
    };

    let mut output = String::new();

    for y in grid_min_y..=grid_max_y {
        for x in grid_min_x..=grid_max_x {
            let pt = Point { x, y };
            let c = if let Some((p1, p2)) = rect {
                let rect_min_x = p1.x.min(p2.x);
                let rect_max_x = p1.x.max(p2.x);
                let rect_min_y = p1.y.min(p2.y);
                let rect_max_y = p1.y.max(p2.y);

                if x >= rect_min_x && x <= rect_max_x && y >= rect_min_y && y <= rect_max_y {
                    'O'
                } else if points.contains(&pt) {
                    '#'
                } else {
                    '.'
                }
            } else if points.contains(&pt) {
                '#'
            } else {
                '.'
            };
            output.push(c);
        }
        output.push('\n');
    }

    output
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

    #[test]
    fn visualize_red_tiles_only() {
        let points = parse_points_set(EXAMPLE);
        let floor = visualize_floor(&points, None, true);
        let expected = "..............
.......#...#..
..............
..#....#......
..............
..#......#....
..............
.........#.#..
..............
";
        assert_eq!(floor, expected);
    }
}
