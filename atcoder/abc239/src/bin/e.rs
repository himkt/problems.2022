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
        Self {
            n,
            graph,
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

    pub fn in_degree(&self, u: usize) -> usize {
        self.graph[u].len()
    }

    pub fn out_degree(&self, u: usize) -> usize {
        self.graph[u].len()
    }

    pub fn connected(&self, u: usize, v: usize) -> bool {
        self.graph[u].iter().filter(|&&(k, _)| v == k).count() > 0
    }
}

pub struct EulerTour {
    graph: Graph,
    l: Vec<usize>,
    r: Vec<usize>,
    t: usize,
}

impl EulerTour {
    const INF: usize = 100_000_000_000_000_000;

    pub fn new(n: usize, graph: Graph) -> Self {
        let l = vec![EulerTour::INF; n];
        let r = vec![EulerTour::INF; n];
        EulerTour { graph, l, r, t: 1 }
    }

    /// Euler tour entrypoint that returns two vectors `(&l, &r)`.
    /// Note that timestamp starts from `1`.
    ///
    /// - `l`: vector indicates the timestamp that visits a node `u` at the first time.
    /// - `r`: vector indicates the timestamp that visits a node `u` at the last time.
    pub fn traverse(&mut self, root: usize) -> (&[usize], &[usize]) {
        self._dfs(root, None);

        for i in 0..self.l.len() {
            if self.r[i] == EulerTour::INF {
                self.r[i] = self.l[i];
            }
        }

        (&self.l, &self.r)
    }

    fn _dfs(&mut self, u: usize, p: Option<usize>) {
        self.l[u] = self.t;
        self.t += 1;

        for i in 0..self.graph.graph[u].len() {
            let (v, _) = self.graph.graph[u][i];
            if p != Some(v) {
                self._dfs(v, Some(u));
                self.r[u] = self.t;
                self.t += 1;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct RMQ {
    v: Vec<(i64, usize)>,
}

impl Default for RMQ {
    fn default() -> Self {
        RMQ::new()
    }
}

impl RMQ {
    const SEQ_LEN: usize = 1<<18;
    const VMIN: i64  = - 1_000_000_000_000_000;
    const INF: usize =   1_000_000_000_000_000;
    const UNIT: (i64, usize) = (RMQ::VMIN, RMQ::INF);

    pub fn new() -> Self {
        Self {
            v: vec![RMQ::UNIT; 2 * RMQ::SEQ_LEN],
        }
    }

    /// Add `value` to i-th element.
    /// 0-origin.
    pub fn update(&mut self, mut index: usize, value: (i64, usize)) {
        index += RMQ::SEQ_LEN;
        self.v[index] = value;

        while index > 0 {
            index /= 2;
            let lv = self.v[index * 2];
            let rv = self.v[index * 2 + 1];
            self.v[index] = lv.max(rv);
        }
    }

    /// Max value on `[l, r)`. Note that it is a half-open interval.
    /// 0-origin.
    pub fn max(&self, l: usize, r: usize) -> (i64, usize) {
        self._max(l, r, 0, RMQ::SEQ_LEN, 1)
    }

    fn _max(&self, ql: usize, qr: usize, sl: usize, sr: usize, pos: usize) -> (i64, usize) {
        if qr <= sl || sr <= ql {
            return RMQ::UNIT;
        }
        if ql <= sl && sr <= qr {
            return self.v[pos];
        }

        let sm = (sl + sr) / 2;
        let lv = self._max(ql, qr, sl, sm, pos * 2);
        let rv = self._max(ql, qr, sm, sr, pos * 2 + 1);
        lv.max(rv)
    }

    pub fn from(v: Vec<i64>) -> Self {
        let mut rsq = RMQ::new();
        for (index, value) in (0..v.len()).zip(v.into_iter()) {
            let idx = RMQ::SEQ_LEN + index;
            rsq.v[idx] = (value, index);
        }

        rsq
    }
}

fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let q: usize = scanner.cin();
    let xs: Vec<i64> = scanner.vec(n);

    let mut graph = Graph::new(n + 1, false);
    for _ in 0..(n - 1) {
        let a: usize = scanner.cin::<usize>() - 1;
        let b: usize = scanner.cin::<usize>() - 1;
        graph.connect_unweighted(a, b);
    }

    let mut solver = EulerTour::new(n, graph);
    let (l, r) = solver.traverse(0);

    let mut rmq = RMQ::new();
    for (u, x) in (0..n).zip(xs) {
        let pair = (x, l[u]);
        rmq.update(pair.1, pair);
    }

    for _ in 0..q {
        let u: usize = scanner.cin::<usize>() - 1;
        let k: usize = scanner.cin();
        let li = l[u];
        let ri = r[u];

        let mut buf = vec![];
        for _ in 0..(k - 1) {
            let mx = rmq.max(li, ri + 1);
            buf.push(mx);
            rmq.update(mx.1, RMQ::UNIT);
        }

        let (ans, _) = rmq.max(li, ri + 1);
        println!("{}", ans);

        for pair in buf {
            rmq.update(pair.1, pair);
        }
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
