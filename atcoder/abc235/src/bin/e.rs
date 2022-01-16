#[derive(Debug,Clone)]
pub struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>
}

#[allow(clippy::needless_range_loop)]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            ranks: vec![1usize; n]
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

        if self.ranks[px] > self.ranks[py] {
            std::mem::swap(&mut px, &mut py);
        }

        if self.ranks[px] == self.ranks[py] {
            self.ranks[py] += 1;
        }

        self.parents[px] = py;
    }
}


const INF: usize = 1001001001001;


#[allow(clippy::collapsible_else_if,clippy::many_single_char_names)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let q: usize = scanner.cin();

    let mut edges = vec![];
    for _ in 0..m {
        let e: Vec<usize> = scanner.vec(3);
        edges.push((e[2], e[0]-1, e[1]-1, INF));
    }

    for query_id in 0..q {
        let e: Vec<usize> = scanner.vec(3);
        edges.push((e[2], e[0]-1, e[1]-1, query_id));
    }

    edges.sort_unstable_by_key(|&(w, _, _, q)| (w, Reverse(q)));

    let mut answers = vec!["No"; q];
    let mut union_find = UnionFind::new(n);

    for (_, source, target, query_id) in edges {
        let a = union_find.find(source);
        let b = union_find.find(target);

        if a == b {
            continue;
        }

        if query_id == INF {
            union_find.unite(source, target);
        }
        else {
            answers[query_id] = "Yes";
        }
    }

    for answer in answers {
        println!("{}", answer);
    }
}


use std::cmp::Reverse;
use std::io::{self, Write};
use std::str::FromStr;
use std::collections::VecDeque;

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
