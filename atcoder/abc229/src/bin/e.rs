fn main() {
    let mut scanner = Scanner::new();

    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut union_find = UnionFind::new(n);

    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let ai: usize = scanner.cin::<usize>() - 1;
        let bi: usize = scanner.cin::<usize>() - 1;
        graph[ai].push(bi);
        graph[bi].push(ai);
    }

    let mut nodes = HashSet::new();
    let mut d = 0;

    let mut hist = VecDeque::new();
    hist.push_front(0);

    for u in (1..n).rev() {
        nodes.insert(u);
        let num_components = nodes.len();  // upper bound

        for &v in graph[u].iter() {
            if !nodes.contains(&v) {
                continue;
            }
            let pu = union_find.parent(u);
            let pv = union_find.parent(v);

            d += if pu != pv { 1 } else { 0 };
            union_find.unite(u, v);
        }
        hist.push_front(num_components - d);
    }

    while let Some(ans) = hist.pop_front() {
        println!("{}", ans);
    }
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

use std::io::{self, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashSet};

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

    fn usize1(&mut self) -> usize {
        self.cin::<usize>() - 1
    }

    fn chars(&mut self) -> Vec<char> {
        self.cin::<String>().chars().collect()
    }

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

#[allow(dead_code)]
fn flush() {
    std::io::stdout().flush().unwrap();
}
