use crate::shared::{Point, parse_points_set, parse_points_vec, point_in_polygon};
use crate::visualize_floor;
use std::collections::HashSet;

/// Part 2: largest rectangle with red corners, entirely within the red/green polygon.
/// Area includes boundaries. Returns max area as u64.
pub fn solve_part2(input: &str) -> u64 {
    solve_part2_with_options(input, false)
}

/// Same as `solve_part2` but can emit visualization of the best rectangle found.
/// Visualization prints to stderr and is skipped if the grid exceeds 100×100.
pub fn solve_part2_with_options(input: &str, visualize: bool) -> u64 {
    let red_vec = parse_points_vec(input);
    if red_vec.len() < 2 {
        return 0;
    }
    let red_set: HashSet<Point> = parse_points_set(input);

    // Build polygon edges (axis-aligned, in given order, closed)
    let edges: Vec<(Point, Point)> = red_vec
        .iter()
        .enumerate()
        .map(|(i, &a)| {
            let b = red_vec[(i + 1) % red_vec.len()];
            (a, b)
        })
        .collect();

    let mut max_area: u64 = 0;
    let mut max_rect: Option<(Point, Point)> = None;

    for i in 0..red_vec.len() {
        for j in i + 1..red_vec.len() {
            let p1 = red_vec[i];
            let p2 = red_vec[j];
            if p1.x == p2.x || p1.y == p2.y {
                continue; // degenerate rectangle
            }

            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);

            let c3 = Point { x: min_x, y: max_y };
            let c4 = Point { x: max_x, y: min_y };

            if !point_in_polygon(c3, &red_vec) || !point_in_polygon(c4, &red_vec) {
                continue;
            }
            if crosses_interior(min_x, max_x, min_y, max_y, &edges) {
                continue;
            }

            let width = (max_x - min_x + 1) as u64;
            let height = (max_y - min_y + 1) as u64;
            let area = width * height;

            if area > max_area {
                max_area = area;
                max_rect = Some((p1, p2));

                if visualize {
                    eprintln!(
                        "\n[Part2] New maximum: area={area} corners=({},{}) to ({},{}) dim={}×{}",
                        p1.x, p1.y, p2.x, p2.y, width, height
                    );
                    let grid_w = (max_x - min_x + 1).abs();
                    let grid_h = (max_y - min_y + 1).abs();
                    if grid_w <= 100 && grid_h <= 100 {
                        let viz = visualize_floor(&red_set, max_rect, false);
                        eprintln!("{viz}");
                    } else {
                        eprintln!(
                            "[Part2] Grid too large to visualize ({}×{})",
                            grid_w, grid_h
                        );
                    }
                }
            }
        }
    }

    if visualize {
        if let Some((a, b)) = max_rect {
            eprintln!(
                "[Part2] Final rectangle: ({},{}) to ({},{}) area={}",
                a.x, a.y, b.x, b.y, max_area
            );
        }
        eprintln!("[Part2] Final answer: {max_area}");
    }

    max_area
}

/// Returns true if any polygon edge crosses the open interior of the rectangle.
fn crosses_interior(rx1: i64, rx2: i64, ry1: i64, ry2: i64, edges: &[(Point, Point)]) -> bool {
    for &(a, b) in edges {
        if a.x == b.x {
            // vertical edge at x = a.x
            let x = a.x;
            if x > rx1 && x < rx2 {
                let ey1 = a.y.min(b.y);
                let ey2 = a.y.max(b.y);
                if intervals_overlap_open(ey1, ey2, ry1, ry2) {
                    return true;
                }
            }
        } else if a.y == b.y {
            // horizontal edge at y = a.y
            let y = a.y;
            if y > ry1 && y < ry2 {
                let ex1 = a.x.min(b.x);
                let ex2 = a.x.max(b.x);
                if intervals_overlap_open(ex1, ex2, rx1, rx2) {
                    return true;
                }
            }
        }
    }
    false
}

/// Check if (a1,a2) overlaps (b1,b2) on open intervals.
fn intervals_overlap_open(a1: i64, a2: i64, b1: i64, b2: i64) -> bool {
    let lo = a1.max(b1);
    let hi = a2.min(b2);
    hi > lo
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
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE), 24);
    }
}
