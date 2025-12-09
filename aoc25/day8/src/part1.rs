use crate::shared::{Dsu, dist2, parse};
use std::cmp::Reverse;

/// Solve Part 1 with a configurable number of processed pairs.
pub fn solve_part1_with_limit(input: &str, pair_limit: usize) -> u128 {
    let pts = parse(input);
    let n = pts.len();
    if n == 0 {
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
    let mut processed = 0usize;

    for (_, a, b) in edges.into_iter() {
        if dsu.find(a) != dsu.find(b) {
            dsu.union(a, b);
        }
        processed += 1;
        if processed == pair_limit {
            break;
        }
    }

    let mut sizes = dsu.component_sizes();
    sizes.sort_by_key(|s| Reverse(*s));
    let a = *sizes.first().unwrap_or(&1);
    let b = *sizes.get(1).unwrap_or(&1);
    let c = *sizes.get(2).unwrap_or(&1);
    a * b * c
}

/// Part 1: default limit 1000.
pub fn solve_part1(input: &str) -> u128 {
    solve_part1_with_limit(input, 1000)
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
    fn example_after_10_pairs() {
        assert_eq!(solve_part1_with_limit(EXAMPLE, 10), 40);
    }
}
