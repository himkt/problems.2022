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
    let mut x: Vec<i64> = vec![0; n];
    let mut y: Vec<i64> = vec![0; n];
    for i in 0..n {
        x[i] = scanner.cin::<i64>();
        y[i] = scanner.cin::<i64>();
    }

    if n == 1 {
        println!("1");
        return;
    }

    let mut pqs = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let p1 = x[i] - x[j];
            let q1 = y[i] - y[j];
            pqs.insert((p1, q1));

            let p2 = x[j] - x[i];
            let q2 = y[j] - y[i];
            pqs.insert((p2, q2));
        }
    }

    let mut ans = 1_000_000;
    for (p, q) in pqs {
        let mut union_find = UnionFind::new(n);
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                if x[i] - p == x[j] && y[i] - q == y[j] {
                    union_find.unite(i, j);
                }
            }
        }

        let parents: HashSet<usize> = (0..n)
            .map(|u| union_find.parent(u))
            .collect();
        ans = ans.min(parents.len());
    }

    println!("{}", ans);
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
