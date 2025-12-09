use crate::shared::{Point, parse_points_vec, point_in_polygon};

/// Part 2: largest rectangle with red corners, entirely within the red/green polygon.
/// Area includes boundaries. Returns max area as u64.
pub fn solve_part2(input: &str) -> u64 {
    let red = parse_points_vec(input);
    if red.len() < 2 {
        return 0;
    }

    // Build polygon edges (axis-aligned, in given order, closed)
    let edges: Vec<(Point, Point)> = red
        .iter()
        .enumerate()
        .map(|(i, &a)| {
            let b = red[(i + 1) % red.len()];
            (a, b)
        })
        .collect();

    let mut max_area: u64 = 0;

    for i in 0..red.len() {
        for j in i + 1..red.len() {
            let p1 = red[i];
            let p2 = red[j];
            if p1.x == p2.x || p1.y == p2.y {
                // degenerate rectangle (zero area)
                continue;
            }

            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);

            // Other two corners
            let c3 = Point { x: min_x, y: max_y };
            let c4 = Point { x: max_x, y: min_y };

            // Both opposite corners are red by construction; check other corners are inside/on polygon
            if !point_in_polygon(c3, &red) || !point_in_polygon(c4, &red) {
                continue;
            }

            // Check polygon edges do not pass through rectangle interior
            if crosses_interior(min_x, max_x, min_y, max_y, &edges) {
                continue;
            }

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
        // Expected 24 per description
        assert_eq!(solve_part2(EXAMPLE), 24);
    }
}
