#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut cnt = HashMap::new();
    let mut num2id = BTreeMap::new();
    for i in 0..n {
        let ai = a[i];
        let k: usize = num2id.len();
        let &mut aid = num2id.entry(ai).or_insert(k);
        *cnt.entry(aid).or_insert(0) += 1;
    }
    debug!("cnt={:?}", cnt);

    let mut union_find = UnionFind::new(n);
    for (&num, &id) in num2id.iter() {
        let nnum = (num + 1) % m;
        if !num2id.contains_key(&nnum) {
            continue
        }
        let nid = num2id[&nnum];
        debug!("unite {} {} ({}, {})", id, nid, num, nnum);
        union_find.unite(id, nid);
    }

    let mut sum_by_group = HashMap::new();
    for (num, &id) in num2id.iter() {
        let group_id = union_find.parent(id);
        *sum_by_group.entry(group_id).or_insert(0) += num * cnt[&id];
    }

    debug!("{:?}", sum_by_group);
    let sum = a.iter().sum::<usize>();
    debug!("sum={}", sum);
    let maxdiff = sum_by_group.values().max().unwrap();
    let ans = sum - maxdiff;
    println!("{}", ans);
}

use std::{io::Write, collections::{HashMap, BTreeMap}};
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
