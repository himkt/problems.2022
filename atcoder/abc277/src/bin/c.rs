#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut union_find = UnionFind::new(2 * n + 1);

    let mut floor2id = HashMap::new();
    for _ in 0..n {
        let ai: usize = scanner.cin();
        let bi: usize = scanner.cin();

        let k: usize = floor2id.len();
        floor2id.entry(ai).or_insert(k);
        let k: usize = floor2id.len();
        floor2id.entry(bi).or_insert(k);

        let aid = floor2id[&ai];
        let bid = floor2id[&bi];
        debug!("unite {} {}", aid, bid);
        union_find.unite(aid, bid);
    }

    debug!("{:?}", floor2id);

    if let Some(&first_floor_id) = floor2id.get(&1) {
        debug!("first={}", first_floor_id);
        let group_id = union_find.parent(first_floor_id);
        let mut ans = 1;
        for (&floor, &id) in floor2id.iter() {
            debug!("floor={}, id={}", floor, id);
            if union_find.parent(id) != group_id {
                continue;
            }
            ans = ans.max(floor);
        }
        println!("{}", ans);
    }
    else {
        println!("1");
    }
}

use std::{io::Write, collections::HashMap};
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

#[macro_export]
macro_rules! debug {
    () => {
        #[cfg(debug_assertions)]
        println!();
    };
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}

#[macro_export]
macro_rules! ndarray {
    ($x:expr;) => { $x };
    ($x:expr; $size:expr $( , $rest:expr )*) => {
        vec![ndarray!($x; $($rest),*); $size]
    };
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
