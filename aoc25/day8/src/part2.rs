use crate::shared::{Dsu, dist2, parse};

/// Part 2: connect until a single circuit; return product of X coords of the last edge used.
pub fn solve_part2(input: &str) -> i128 {
    let pts = parse(input);
    let n = pts.len();

    if n <= 1 {
        return 0;
    }

    let mut edges = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            edges.push((dist2(pts[i], pts[j]), i, j));
        }
    }

    edges.sort_by(|(d1, i1, j1), (d2, i2, j2)| d1.cmp(d2).then(i1.cmp(i2)).then(j1.cmp(j2)));

    let mut dsu = Dsu::new(n);
    let mut components = n;
    let mut last_product: i128 = 0;

    for (_, a, b) in edges.into_iter() {
        if dsu.union(a, b) {
            components -= 1;
            last_product = pts[a].x as i128 * pts[b].x as i128;
            if components == 1 {
                break;
            }
        }
    }

    last_product
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn example_part2_last_edge_product() {
        assert_eq!(solve_part2(EXAMPLE), 25_272);
    }
}
