#[derive(Debug, Clone)]
pub struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

#[allow(clippy::needless_range_loop)]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            sizes: vec![1usize; n],
        }
    }

    pub fn parent(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            x
        } else {
            self.parents[x] = self.parent(self.parents[x]);
            self.parents[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let mut px = self.parent(x);
        let mut py = self.parent(y);

        if px == py {
            return;
        }

        if self.sizes[px] < self.sizes[py] {
            std::mem::swap(&mut px, &mut py);
        }

        self.sizes[px] += self.sizes[py];
        self.parents[py] = px;
    }

    pub fn size(&mut self, x: usize) -> usize {
        let x = self.parent(x);
        self.sizes[x]
    }
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let sx: i64 = scanner.cin();
    let sy: i64 = scanner.cin();
    let tx: i64 = scanner.cin();
    let ty: i64 = scanner.cin();

    let circles: Vec<(i64, i64, i64)> = (0..n)
        .map(|_| {
            let x: i64 = scanner.cin();
            let y: i64 = scanner.cin();
            let r: i64 = scanner.cin();
            (x, y, r)
        })
        .collect();

    let mut union_find = UnionFind::new(n);
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            let (x1, y1, r1) = circles[i];
            let (x2, y2, r2) = circles[j];
            let dpow = (x2 - x1).pow(2) + (y2 - y1).pow(2);

            let rmax = r1.max(r2);
            let rmin = r1.min(r2);
            let l = rmax - rmin;
            let r = rmax + rmin;

            if l * l <= dpow && dpow <= r * r {
                if union_find.parent(i) != union_find.parent(j) {
                    union_find.unite(i, j);
                }
            }
        }
    }

    let mut s_candidates = vec![];
    for i in 0..n {
        let (ox, oy, r) = circles[i];
        let (x, y) = (sx, sy);

        let dpow = (x - ox).pow(2) + (y - oy).pow(2);
        if dpow == r.pow(2) {
            s_candidates.push(i);
        }
    }

    let mut t_candidates = vec![];
    for i in 0..n {
        let (ox, oy, r) = circles[i];
        let (x, y) = (tx, ty);

        let dpow = (x - ox).pow(2) + (y - oy).pow(2);
        if dpow == r.pow(2) {
            t_candidates.push(i);
        }
    }

    for si in s_candidates {
        for &ti in t_candidates.iter() {
            if union_find.parent(si) == union_find.parent(ti) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}

use std::io::Write; pub fn flush() { std::io::stdout().flush().unwrap(); }
pub struct Scanner { buffer: std::collections::VecDeque<String>, buf: String }
impl Scanner {
    pub fn new() -> Self { Scanner { buffer: std::collections::VecDeque::new(), buf: String::new() } }
    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        if !self.buffer.is_empty() { return self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap(); }
        self.buf.truncate(0); std::io::stdin().read_line(&mut self.buf).ok();
        self.buf.to_owned().split_whitespace().for_each(|x| self.buffer.push_back(String::from(x)));
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> { (0..n).map(|_| self.cin()).collect() }
}
