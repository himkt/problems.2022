#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let k: usize = scanner.cin();

    let mut friends: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut blocks: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut union_find = UnionFind::new(n);

    for _ in 0..m {
        let a: usize = scanner.cin::<usize>() - 1;
        let b: usize = scanner.cin::<usize>() - 1;
        union_find.unite(a, b);

        friends.entry(a).or_insert_with(HashSet::new);
        friends.entry(a)
            .and_modify(|ent| { let _ = (*ent).insert(b); });

        friends.entry(b).or_insert_with(HashSet::new);
        friends.entry(b)
            .and_modify(|ent| { let _ = (*ent).insert(a); });
    }

    for _ in 0..k {
        let c: usize = scanner.cin::<usize>() - 1;
        let d: usize = scanner.cin::<usize>() - 1;

        blocks.entry(c).or_insert_with(HashSet::new);
        blocks.entry(c)
            .and_modify(|ent| { let _ = (*ent).insert(d); });

        blocks.entry(d).or_insert_with(HashSet::new);
        blocks.entry(d)
            .and_modify(|ent| { let _ = (*ent).insert(c); });
    }

    for u in 0..n {
        let mut ret = union_find.size(u);

        if friends.contains_key(&u) {
            ret -= friends[&u].len();
        }

        if blocks.contains_key(&u) {
            for &v in blocks[&u].iter() {
                if union_find.parent(u) == union_find.parent(v) {
                    ret -= 1;
                }
            }
        }

        // -1 for excluding `u` itself.
        println!("{}", ret - 1);
    }
}

use std::{io::Write, collections::{HashSet, HashMap}};
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
