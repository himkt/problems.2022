#[derive(Debug,Clone)]
pub struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>
}

#[allow(clippy::needless_range_loop)]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            sizes: vec![1usize; n]
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            x
        }
        else {
            self.parents[x] = self.find(self.parents[x]);
            self.parents[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let mut px = self.find(x);
        let mut py = self.find(y);

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
        let x = self.find(x);
        self.sizes[x]
    }
}


fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let mut edges = vec![];

    for _ in 0..n-1 {
        let u: usize = scanner.cin();
        let v: usize = scanner.cin();
        let w: usize = scanner.cin();
        edges.push((u-1, v-1, w));
    }

    edges.sort_unstable_by_key(|&(_, _, cost)| cost);

    let mut ws: HashMap<usize, usize> = HashMap::new();
    let mut union_find = UnionFind::new(n);
    for (u, v, w) in edges {
        let num_u = union_find.size(u);
        let num_v = union_find.size(v);
        *ws.entry(w).or_insert(0) += num_u * num_v;
        union_find.unite(u, v);
    }

    let mut ans = 0;
    for (w, freq) in ws {
        ans += w*freq;
    }

    println!("{}", ans);
}


use std::collections::{VecDeque, HashMap};
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
