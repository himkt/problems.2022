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
    let a: usize = scanner.cin();
    let b: usize = scanner.cin();

    let x: Vec<usize> = scanner.vec(n);
    let mut es = vec![];
    for i in 1..n {
        let c = a * (x[i] - x[i-1]);
        es.push((c, (i-1, i)));
    }

    es.sort_by_key(|&(c, _)| c);

    let mut ans = 0;
    let mut union_find = UnionFind::new(n);
    for (c, (s, t)) in es {
        if c < b {
            union_find.unite(s, t);
            ans += c;
        }
    }

    let mut parents: HashSet<usize> = HashSet::new();
    for i in 0..n {
        parents.insert(union_find.find(i));
    }

    ans += b * (parents.len() - 1);
    println!("{}", ans);
}


use std::collections::{VecDeque, HashSet};
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
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
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

#[macro_export]
macro_rules! trace {
    ($x:expr) => {
        #[cfg(debug_assertions)]
        eprintln!(">>> {} = {:?}", stringify!($x), $x)
    };
    ($($xs:expr),*) => { trace!(($($xs),*)) }
}
