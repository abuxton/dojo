#[derive(Clone, Copy)]
pub struct Pt {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

pub fn parse(input: &str) -> Vec<Pt> {
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

// Squared Euclidean distance (clippy-clean via abs_diff)
pub fn dist2(a: Pt, b: Pt) -> u128 {
    let dx = a.x.abs_diff(b.x) as u128;
    let dy = a.y.abs_diff(b.y) as u128;
    let dz = a.z.abs_diff(b.z) as u128;
    dx * dx + dy * dy + dz * dz
}

pub struct Dsu {
    p: Vec<usize>,
    sz: Vec<u128>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        Self {
            p: (0..n).collect(),
            sz: vec![1; n],
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if self.p[x] != x {
            self.p[x] = self.find(self.p[x]);
        }
        self.p[x]
    }
    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.sz[ra] < self.sz[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.p[rb] = ra;
        self.sz[ra] += self.sz[rb];
        true
    }
    pub fn component_sizes(&mut self) -> Vec<u128> {
        let mut out = Vec::new();
        for i in 0..self.p.len() {
            if self.find(i) == i {
                out.push(self.sz[i]);
            }
        }
        out
    }
}
