#[derive(Clone, Debug)]
pub struct Graph {
    pub n: usize,
    pub graph: Vec<Vec<(usize, usize)>>,
    pub in_degrees: Vec<usize>,
    pub out_degrees: Vec<usize>,
    directed: bool,
}


impl Graph {
    pub fn new(n: usize, directed: bool) -> Self {
        let graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
        let in_degrees = vec![0; n];
        let out_degrees = vec![0; n];
        Self { n, graph, in_degrees, out_degrees, directed }
    }

    pub fn connect(&mut self, from: usize, to: usize, weight: usize) {
        self.graph[from].push((to, weight));
        self.out_degrees[from] += 1;
        self.in_degrees[to] += 1;

        if !self.directed {
            self.graph[to].push((from, weight));
            self.out_degrees[to] += 1;
            self.in_degrees[from] += 1;
        }
    }

    pub fn connect_unweighted(&mut self, from: usize, to: usize) {
        self.graph[from].push((to, 1));
        self.out_degrees[from] += 1;
        self.in_degrees[to] += 1;

        if !self.directed {
            self.graph[to].push((from, 1));
            self.out_degrees[to] += 1;
            self.in_degrees[from] += 1;
        }
    }

    pub fn in_degree(&self, u: usize) -> usize {
        self.graph[u].len()
    }

    pub fn out_degree(&self, u: usize) -> usize {
        self.graph[u].len()
    }

    pub fn connected(&self, u: usize, v: usize) -> bool {
        self.graph[u].iter().filter(|&&(k,_)| v == k).count() > 0
    }
}


#[derive(Debug, Clone)]
pub struct Lowlink {
    graph: Graph,
    used: Vec<bool>,
    ord: Vec<usize>,
    low: Vec<usize>,
    bridges: Vec<(usize, usize)>,
}


#[allow(clippy::needless_range_loop)]
impl Lowlink {
    pub fn new(graph: Graph) -> Self {
        let n: usize = graph.n;
        let used = vec![false; n];
        let ord: Vec<usize> = vec![0; n];
        let low: Vec<usize> = vec![0; n];
        let bridges: Vec<(usize, usize)> = vec![];

        Self {
            graph,
            used,
            ord,
            low,
            bridges,
        }
    }

    pub fn search(&mut self) {
        let mut k = 0;
        for u in 0..self.graph.n {
            if self.used[u] {
                continue;
            }
            k = self.dfs(u, k, None);
        }
    }

    pub fn dfs(&mut self, u: usize, mut k: usize, p: Option<usize>) -> usize {
        self.used[u] = true;

        self.ord[u] = k;
        self.low[u] = k;
        k += 1;

        for i in 0..self.graph.graph[u].len() {
            let (v, _) = self.graph.graph[u][i];

            if !self.used[v] {
                k = self.dfs(v, k, Some(u));
                self.low[u] = self.low[u].min(self.low[v]);

                if self.ord[u] < self.low[v] {
                    self.bridges.push((u.min(v), u.max(v)));
                }
            } else if p.is_some() && v != p.unwrap() {
                self.low[u] = self.low[u].min(self.ord[v]);
            }
        }

        k
    }

    pub fn num_bridges(&self) -> usize {
        self.bridges.len()
    }
}


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut graph = Graph::new(n, false);
    for _ in 0..m {
        let a: usize = scanner.cin::<usize>() - 1;
        let b: usize = scanner.cin::<usize>() - 1;
        graph.connect_unweighted(a, b);
    }

    let mut lowlink = Lowlink::new(graph);
    lowlink.search();

    println!("{}", lowlink.num_bridges());
}


use std::collections::VecDeque;
use std::io::{self, Write};
use std::str::FromStr;

pub struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
impl Scanner {
    fn new() -> Self {
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
    }

    pub fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

pub fn flush() {
    std::io::stdout().flush().unwrap();
}
