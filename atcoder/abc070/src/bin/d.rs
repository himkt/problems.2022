pub struct GraphBuilder {
    pub graph: Vec<Vec<usize>>,
    directed: bool,
}


pub struct WeightedGraphBuilder {
    pub graph: Vec<Vec<(usize, usize)>>,
    directed: bool,
}


impl GraphBuilder {
    pub fn new(n: usize, directed: bool) -> Self {
        let graph: Vec<Vec<usize>> = vec![vec![]; n];
        Self { graph, directed }
    }

    pub fn connect(&mut self, from: usize, to: usize) {
        self.graph[from].push(to);
        if !self.directed {
            self.graph[to].push(from);
        }
    }
}


impl WeightedGraphBuilder {
    pub fn new(n: usize, directed: bool) -> Self {
        let graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
        Self { graph, directed }
    }

    pub fn connect(&mut self, from: usize, to: usize, weight: usize) {
        self.graph[from].push((to, weight));
        if !self.directed {
            self.graph[to].push((from, weight));
        }
    }
}



use std::{collections::BinaryHeap, cmp::Reverse};

const INF: usize = 100_000_000_000_000_000;


#[derive(Debug, Clone)]
pub struct Dijkstra {
    graph: Vec<Vec<(usize, usize)>>,
}


impl Dijkstra {
    pub fn new(graph: Vec<Vec<(usize, usize)>>) -> Self {
        Self { graph }
    }

    pub fn search(&mut self, src: usize) -> Vec<usize> {
        let mut dist = vec![INF; self.graph.len()];
        dist[src] = 0;

        let mut queue = BinaryHeap::new();
        queue.push((Reverse(0), src));

        while let Some((Reverse(current_cost), current_v)) = queue.pop() {
            if dist[current_v] < current_cost {
                continue;
            }

            for &(v, cost) in self.graph[current_v].iter() {
                if dist[v] > current_cost + cost {
                    dist[v] = current_cost + cost;
                    queue.push((Reverse(dist[v]), v));
                }
            }
        }

        dist
    }
}


const ROOT: usize = 0;
const MAX_LOG_V: usize = 30;


pub struct LCA {
    parents: Vec<Vec<usize>>,
    depth: Vec<usize>,
    graph: Vec<Vec<usize>>,
}


impl LCA {
    pub fn new(graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        let parents = vec![vec![ROOT; n]; MAX_LOG_V];
        let depth = vec![ROOT; n];
        LCA { parents, depth, graph }
    }

    pub fn init(&mut self) {
        self.dfs(ROOT, ROOT, 0);

        for k in 0..MAX_LOG_V-1 {
            for v in 0..self.graph.len() {
                self.parents[k+1][v] = self.parents[k][self.parents[k][v]];
            }
        }
    }

    fn dfs(&mut self, v: usize, p: usize, d: usize) {
        self.parents[0][v] = p;
        self.depth[v] = d;

        for i in 0..self.graph[v].len() {
            let nv = self.graph[v][i];

            if nv != p {
                self.dfs(nv, v, d + 1);
            }
        }
    }

    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        if self.depth[u] > self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }

        for k in 0..MAX_LOG_V {
            if (((self.depth[v] - self.depth[u]) >> k) & 1) == 1 {
                v = self.parents[k][v];
            }
        }

        if u == v {
            return u;
        }

        for k in (0..MAX_LOG_V).rev() {
            if self.parents[k][u] != self.parents[k][v] {
                u = self.parents[k][u];
                v = self.parents[k][v];
            }
        }

        self.parents[0][u]
    }
}


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let mut graph_dijkstra = WeightedGraphBuilder::new(n, false);
    let mut graph_lca = GraphBuilder::new(n, false);

    for _ in 0..n-1 {
        let a = scanner.cin::<usize>() - 1;
        let b = scanner.cin::<usize>() - 1;
        let c: usize = scanner.cin();

        graph_dijkstra.connect(a, b, c);
        graph_lca.connect(a, b);
    }

    let mut dijkstra = Dijkstra::new(graph_dijkstra.graph);
    let dist = dijkstra.search(ROOT);

    let mut lca = LCA::new(graph_lca.graph);
    lca.init();

    let q: usize = scanner.cin();
    let k: usize = scanner.cin::<usize>() - 1;
    for _ in 0..q {
        let x = scanner.cin::<usize>() - 1;
        let y = scanner.cin::<usize>() - 1;

        let cx = lca.lca(x, k);
        let cy = lca.lca(y, k);

        let dx = if cx == 0 {
            dist[x] + dist[k]
        }
        else {
            dist[x] + dist[k] - 2 * dist[cx]
        };

        let dy = if cy == 0 {
            dist[y] + dist[k]
        }
        else {
            dist[y] + dist[k] - 2 * dist[cy]
        };

        let ans = dx + dy;
        println!("{}", ans);
    }
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
