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


pub struct TopologicalSort {
    graph: Graph,
}


#[allow(clippy::needless_range_loop)]
impl TopologicalSort {
    pub fn new(graph: Graph) -> Self {
        TopologicalSort { graph }
    }

    pub fn sort(&mut self) -> Vec<usize> {
        let mut ans: Vec<usize> = vec![];
        let mut s = std::collections::BinaryHeap::new();
        let mut degrees = self.graph.in_degrees.clone();

        for v in 0..self.graph.n {
            if degrees[v] == 0 {
                s.push(std::cmp::Reverse(v));
            }
        }

        while let Some(std::cmp::Reverse(v)) = s.pop() {
            ans.push(v);

            for &(nv, _) in self.graph.graph[v].iter() {
                if degrees[nv] == 0 {
                    continue;
                }

                degrees[nv] -= 1;

                if degrees[nv] == 0 {
                    s.push(std::cmp::Reverse(nv));
                }
            }
        }

        if ans.len() == degrees.len() {
            ans
        } else {
            vec![]
        }
    }
}


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut graph = Graph::new(n, true);

    for _ in 0..m {
        let a: usize = scanner.cin();
        let a: usize = a - 1;
        let b: usize = scanner.cin();
        let b: usize = b - 1;

        if graph.connected(a, b) {
            continue;
        }
        graph.connect_unweighted(a, b);
    }

    let mut topological_sorter = TopologicalSort::new(graph);
    let ans = topological_sorter.sort();

    if ans.len() == n {
        let ans: Vec<_> = ans.iter().map(|x| (x+1).to_string()).collect();
        let ans = ans.join(" ");
        println!("{}", ans);
    }
    else {
        println!("-1");
    }

}


use std::collections::{VecDeque};
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
