use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

/// Parse comma-separated x,y coordinates preserving order.
pub fn parse_points_vec(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let (xs, ys) = l
                .split_once(',')
                .unwrap_or_else(|| panic!("bad line (missing comma): {l}"));
            let x = xs.trim().parse::<i64>().expect("bad x coord");
            let y = ys.trim().parse::<i64>().expect("bad y coord");
            Point { x, y }
        })
        .collect()
}

/// Parse into a HashSet (order not preserved).
pub fn parse_points_set(input: &str) -> HashSet<Point> {
    parse_points_vec(input).into_iter().collect()
}

/// Ray-cast point-in-polygon (even-odd), treating boundary as inside.
pub fn point_in_polygon(pt: Point, poly: &[Point]) -> bool {
    if poly.len() < 3 {
        return false;
    }
    // Boundary check: if on any edge, treat as inside.
    for i in 0..poly.len() {
        let a = poly[i];
        let b = poly[(i + 1) % poly.len()];
        if on_segment_inclusive(pt, a, b) {
            return true;
        }
    }

    let mut inside = false;
    let mut j = poly.len() - 1;
    for i in 0..poly.len() {
        let pi = poly[i];
        let pj = poly[j];
        let intersects = (pi.y > pt.y) != (pj.y > pt.y)
            && (pt.x < (pj.x - pi.x) * (pt.y - pi.y) / (pj.y - pi.y).max(1) + pi.x);
        if intersects {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn on_segment_inclusive(p: Point, a: Point, b: Point) -> bool {
    // axis-aligned edges only per problem statement
    if a.x == b.x {
        // vertical
        p.x == a.x && between(p.y, a.y, b.y)
    } else if a.y == b.y {
        // horizontal
        p.y == a.y && between(p.x, a.x, b.x)
    } else {
        false
    }
}

fn between(v: i64, a: i64, b: i64) -> bool {
    (v >= a && v <= b) || (v >= b && v <= a)
}
