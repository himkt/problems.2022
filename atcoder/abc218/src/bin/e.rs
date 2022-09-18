#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let ai: usize = scanner.cin::<usize>() - 1;
        let bi: usize = scanner.cin::<usize>() - 1;
        let ci: i64 = scanner.cin();
        graph[ai].push((bi, ci));
    }

    let mut pooled_edges = vec![];

    let mut union_find = UnionFind::new(n);
    for u in 0..n {
        for &(v, ci) in graph[u].iter() {
            if ci <= 0 {
                union_find.unite(u, v);
            }
            else {
                pooled_edges.push((u, v, ci));
            }
        }
    }

    let mut ans = 0;
    pooled_edges.sort_unstable_by_key(|&(_, _, ci)| ci);
    for (u, v, ci) in pooled_edges {
        let pu = union_find.parent(u);
        let pv = union_find.parent(v);

        if pu != pv {
            union_find.unite(u, v);
        }
        else {
            ans += ci;
        }
    }

    println!("{}", ans);
}

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

use std::collections::VecDeque;
use std::io::{self, Write};
use std::str::FromStr;

#[allow(dead_code)]
struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
#[allow(dead_code)]
impl Scanner {
    fn new() -> Self {
        Self {
            stdin: io::stdin(),
            buffer: VecDeque::new(),
        }
    }

    fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

#[allow(dead_code)]
fn flush() {
    std::io::stdout().flush().unwrap();
}
