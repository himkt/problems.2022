const MAX: usize = 1_000_000_000;
const MIN: usize = 0;

pub struct Traverser {
    pub lefts: Vec<Option<usize>>,
    pub rights: Vec<Option<usize>>,
    pub graph: Vec<Vec<usize>>,
    pub used: Vec<bool>,
}

impl Traverser {
    fn new(
        lefts: Vec<Option<usize>>,
        rights: Vec<Option<usize>>,
        graph: Vec<Vec<usize>>,
    ) -> Self {
        let n: usize = graph.len();
        let used = vec![false; n];
        Traverser { lefts, rights, graph, used }
    }

    fn traverse(&mut self, u: usize) -> (usize, usize) {
        self.used[u] = true;
        //println!("[innter] L: {:?}, R: {:?}", self.lefts, self.rights);
        //println!("[innter] visit: {}, li: {:?}, ri: {:?}", u, self.lefts[u], self.rights[u]);

        if let (Some(l), Some(r)) = (self.lefts[u], self.rights[u]) {
            return (l.min(r), l.max(r));
        }

        let mut lv = MAX;
        let mut rv = MIN;
        for j in 0..self.graph[u].len() {
            let v = self.graph[u][j];
            if self.used[v] {
                continue;
            }
            //println!("  [inner] next: {}", v);
            let (lvi, rvi) = self.traverse(v);
            lv = lv.min(lvi);
            rv = rv.max(rvi);
        }

        self.lefts[u] = Some(lv);
        self.rights[u] = Some(rv);
        (lv, rv)
    }
}

fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut graph = vec![vec![]; n + 1];
    for _ in 0..(n - 1) {
        let u: usize = scanner.cin();
        let v: usize = scanner.cin();
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut node_number = vec![None; n + 1];
    let mut lefts = vec![None; n + 1];
    let mut rights = vec![None; n + 1];

    let mut stack = VecDeque::new();
    let mut used = vec![false; n + 1];
    let mut v = 1;

    stack.push_front(1);
    while let Some(u) = stack.pop_front() {
        used[u] = true;

        let mut num_transitions = 0;
        for &v in graph[u].iter() {
            if used[v] {
                continue;
            }
            stack.push_front(v);
            num_transitions += 1;
        }

        if num_transitions == 0 {
            node_number[u] = Some(v);
            lefts[u] = Some(v);
            rights[u] = Some(v);
            v += 1;
        }
    }

    let mut traverser = Traverser::new(lefts, rights, graph);
    traverser.traverse(1);

    for i in 1..=n {
        let l = traverser.lefts[i].unwrap();
        let r = traverser.rights[i].unwrap();
        println!("{} {}", l, r);
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
