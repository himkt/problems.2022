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

fn position2id(i: i64, j: i64) -> usize {
    10000 * i as usize + j as usize
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut xs = vec![];
    let mut ys = vec![];
    for _ in 0..n {
        let x: i64 = scanner.cin::<i64>() + 2000;
        let y: i64 = scanner.cin::<i64>() + 2000;
        xs.push(x);
        ys.push(y);
    }

    let mut set = HashSet::new();
    for i in 0..n {
        let id = position2id(xs[i], ys[i]);
        set.insert(id);
    }

    let dis = vec![-1, -1, 0, 0, 1, 1];
    let djs = vec![-1, 0, -1, 1, 0, 1];

    let mut union_find = UnionFind::new(30_003_001);
    // for i in 0..n {
    //     let id = position2id(xs[i], ys[i]);
    //     println!("[ini] ({}, {}) => parent {}", xs[i], ys[i], union_find.parent(id));
    // }

    for i in 0..n {
        let center = position2id(xs[i], ys[i]);
        // println!("({}, {}) => {}", xs[i], ys[i], center);
        for k in 0..6 {
            let di = dis[k];
            let dj = djs[k];
            let ni = xs[i] + di;
            let nj = ys[i] + dj;
            let surrounding = position2id(ni, nj);
            if !set.contains(&surrounding) {
                continue;
            }
            // println!("s ({}, {}) => {}", ni, nj, surrounding);
            union_find.unite(center, surrounding);
        }
    }

    let mut parents = HashSet::new();
    for i in 0..n {
        let id = position2id(xs[i], ys[i]);
        parents.insert(union_find.parent(id));
    }

    println!("{}", parents.len());
}

use std::{io::Write, collections::HashSet};
pub struct Scanner {
    buffer: std::collections::VecDeque<String>,
    buf: String,
}
#[allow(clippy::new_without_default)]
impl Scanner {
    pub fn new() -> Self {
        Scanner {
            buffer: std::collections::VecDeque::new(),
            buf: String::new(),
        }
    }
    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        if !self.buffer.is_empty() {
            return self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap();
        }
        self.buf.truncate(0);
        std::io::stdin().read_line(&mut self.buf).ok();
        self.buf
            .to_owned()
            .split_whitespace()
            .for_each(|x| self.buffer.push_back(String::from(x)));
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
    pub fn flush(&self) {
        std::io::stdout().flush().unwrap();
    }
}
