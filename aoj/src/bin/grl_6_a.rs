#[derive(Clone, Debug)]
pub struct Graph {
    pub n: usize,
    pub graph: Vec<Vec<(usize, usize)>>,
    pub rev: Vec<Vec<usize>>,
    pub in_degrees: Vec<usize>,
    pub out_degrees: Vec<usize>,
    pub directed: bool,
}

impl Graph {
    pub fn new(n: usize, directed: bool) -> Self {
        let graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
        let in_degrees = vec![0; n];
        let out_degrees = vec![0; n];
        let rev = vec![vec![]; n];
        Self {
            n,
            graph,
            rev,
            in_degrees,
            out_degrees,
            directed,
        }
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

    pub fn connect_with_residual(&mut self, from: usize, to: usize, weight: usize) {
        assert!(self.directed, "connect_with_residual only works in directed graph.");

        self.graph[from].push((to, weight));
        self.out_degrees[from] += 1;
        self.in_degrees[to] += 1;

        self.graph[to].push((from, 0));
        self.out_degrees[to] += 1;
        self.in_degrees[from] += 1;

        self.rev[from].push(self.graph[to].len() - 1);
        self.rev[to].push(self.graph[from].len() - 1);
    }

    pub fn in_degree(&self, u: usize) -> usize {
        self.graph[u].len()
    }

    pub fn out_degree(&self, u: usize) -> usize {
        self.graph[u].len()
    }

    pub fn connected(&self, u: usize, v: usize) -> bool {
        self.graph[u].iter().filter(|&(k, _)| &v == k).count() > 0
    }
}

pub struct FordFullkerson {
    pub graph: Graph,
    pub used: Vec<bool>,
    pub flow: usize,
}

impl FordFullkerson {
    const INF: usize = 1001001001;

    pub fn new(graph: Graph) -> Self {
        let used = vec![false; graph.n];
        let flow = FordFullkerson::INF;
        FordFullkerson { graph, used, flow }
    }

    pub fn dfs(&mut self, v: usize, t: usize, f: usize) -> usize {
        if v == t {
            return f;
        }

        self.used[v] = true;
        for i in 0..self.graph.graph[v].len() {
            let (u, cap) = self.graph.graph[v][i];
            let rev = self.graph.rev[v][i];
            if !self.used[u] && cap > 0 {
                let d = self.dfs(u, t, f.min(cap));

                if d > 0 {
                    self.graph.graph[v][i].1 -= d;
                    self.graph.graph[u][rev].1 += d;
                    return d;
                }
            }
        }

        0
    }

    pub fn solve(&mut self, s: usize, t: usize) -> usize {
        let mut flow = 0;
        loop {
            self.used = vec![false; self.graph.n];
            let f: usize = self.dfs(s, t, FordFullkerson::INF);
            if f == 0 {
                return flow;
            }
            flow += f;
        }
    }
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let nv: usize = scanner.cin();
    let ne: usize = scanner.cin();

    let mut graph = Graph::new(nv, true);
    for _ in 0..ne {
        let v0 = scanner.cin();
        let v1 = scanner.cin();
        let c = scanner.cin();
        graph.connect_with_residual(v0, v1, c);
    }

    let mut solver = FordFullkerson::new(graph);
    println!("{}", solver.solve(0, nv - 1));
}

use std::io::Write;
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
