use std::cmp::Reverse;

#[derive(Clone, Copy)]
struct Pt {
    x: i64,
    y: i64,
    z: i64,
}

fn parse(input: &str) -> Vec<Pt> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let nums: Vec<i64> = l
                .split(',')
                .map(|s| s.trim().parse::<i64>().expect("bad coord"))
                .collect();
            Pt {
                x: nums[0],
                y: nums[1],
                z: nums[2],
            }
        })
        .collect()
}

// Squared Euclidean distance (using abs_diff, clippy-clean)
fn dist2(a: Pt, b: Pt) -> u128 {
    let dx = a.x.abs_diff(b.x) as u128;
    let dy = a.y.abs_diff(b.y) as u128;
    let dz = a.z.abs_diff(b.z) as u128;
    dx * dx + dy * dy + dz * dz
}

struct Dsu {
    p: Vec<usize>,
    sz: Vec<u128>,
}
impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            p: (0..n).collect(),
            sz: vec![1; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.p[x] != x {
            self.p[x] = self.find(self.p[x]);
        }
        self.p[x]
    }
    fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return;
        }
        if self.sz[ra] < self.sz[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.p[rb] = ra;
        self.sz[ra] += self.sz[rb];
    }
    fn component_sizes(&mut self) -> Vec<u128> {
        let mut out = Vec::new();
        for i in 0..self.p.len() {
            if self.find(i) == i {
                out.push(self.sz[i]);
            }
        }
        out
    }
}

/// Solve with a configurable number of processed pairs (for examples/tests).
pub fn solve_with_k(input: &str, k: usize) -> u128 {
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
    // Deterministic ordering: distance, then i, then j (ties matter)
    edges.sort_by(|(d1, i1, j1), (d2, i2, j2)| d1.cmp(d2).then(i1.cmp(i2)).then(j1.cmp(j2)));

    let mut dsu = Dsu::new(n);
    let mut processed = 0usize;

    for (_, a, b) in edges.into_iter() {
        if dsu.find(a) != dsu.find(b) {
            dsu.union(a, b);
        }
        processed += 1;
        if processed == k {
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

/// Part 1: process the 1000 closest pairs and return product of the three largest circuits.
pub fn solve(input: &str) -> u128 {
    solve_with_k(input, 1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
162,817,812
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
        assert_eq!(solve_with_k(EXAMPLE, 10), 40);
    }
}
