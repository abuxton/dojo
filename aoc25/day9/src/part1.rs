use crate::shared::{Point, parse};
use std::collections::HashSet;

/// Find the largest rectangle with opposite corners both being red tiles.
/// Area includes the boundary (inclusive of both endpoints).
///
/// If `visualize` is true, prints the grid with each new maximum rectangle found.
pub fn solve_part1_with_options(input: &str, visualize: bool) -> u64 {
    let points = parse(input);

    if points.len() < 2 {
        return 0;
    }

    let mut max_area: u64 = 0;
    let mut max_rect: Option<(Point, Point)> = None;
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

            // Calculate area (inclusive of boundaries)
            let width = (max_x - min_x + 1) as u64;
            let height = (max_y - min_y + 1) as u64;
            let area = width * height;

            if area > max_area {
                max_area = area;
                max_rect = Some((p1, p2));

                if visualize {
                    eprintln!("\n=== New maximum found: {} ===", area);
                    eprintln!("Corners: ({},{}) and ({},{})", p1.x, p1.y, p2.x, p2.y);
                    eprintln!("Dimensions: {} × {}", width, height);

                    // Only visualize if grid is reasonably sized
                    let grid_width = (max_x - min_x + 1).abs();
                    let grid_height = (max_y - min_y + 1).abs();
                    if grid_width <= 100 && grid_height <= 100 {
                        let viz = visualize_floor(&points, max_rect, false);
                        eprintln!("\n{}", viz);
                    } else {
                        eprintln!(
                            "(Grid too large to visualize: {}×{})",
                            grid_width, grid_height
                        );
                    }
                }
            }
        }
    }

    if visualize && max_rect.is_some() {
        eprintln!("\n=== Final answer: {} ===", max_area);
    }

    max_area
}

/// Find the largest rectangle (no visualization).
pub fn solve_part1(input: &str) -> u64 {
    solve_part1_with_options(input, false)
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
    fn example_with_visualization() {
        let result = solve_part1_with_options(EXAMPLE, true);
        assert_eq!(result, 50);
    }

    #[test]
    fn visualize_red_tiles_only() {
        let points = parse(EXAMPLE);
        let floor = visualize_floor(&points, None, true);
        println!("\nRed tiles only:\n{}", floor);

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

    #[test]
    fn visualize_with_small_rectangle() {
        let points = parse(EXAMPLE);
        let p1 = Point { x: 2, y: 5 };
        let p2 = Point { x: 9, y: 7 };
        let floor = visualize_floor(&points, Some((p1, p2)), true);
        println!("\nRectangle 2,5 to 9,7 (area 24):\n{}", floor);

        let expected = "..............
.......#...#..
..............
..#....#......
..............
..OOOOOOOO....
..OOOOOOOO....
..OOOOOOOO.#..
..............
";
        assert_eq!(floor, expected);
    }

    #[test]
    fn visualize_with_largest_rectangle() {
        let points = parse(EXAMPLE);
        let p1 = Point { x: 2, y: 1 };
        let p2 = Point { x: 11, y: 5 };
        let floor = visualize_floor(&points, Some((p1, p2)), true);
        println!("\nLargest rectangle 2,1 to 11,5 (area 50):\n{}", floor);

        let expected = "..............
..OOOOOOOOOO..
..OOOOOOOOOO..
..OOOOOOOOOO..
..OOOOOOOOOO..
..OOOOOOOOOO..
..............
.........#.#..
..............
";
        assert_eq!(floor, expected);
    }

    #[test]
    fn large_coordinate_values() {
        // Test with large input values
        let input = "97498,50350
2103,50332";
        let result = solve_part1(input);
        // (97498-2103+1) * (50350-50332+1) = 95396 * 19 = 1,812,524
        assert_eq!(result, 1_812_524);
    }
}
