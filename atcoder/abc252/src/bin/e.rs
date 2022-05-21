use std::{collections::BinaryHeap, cmp::Reverse};

const INF: usize = 100_000_000_000_000;

#[derive(Debug, Clone)]
pub struct Dijkstra {
    graph: Vec<Vec<(usize, usize)>>,
    backptr: Vec<usize>,
}

impl Dijkstra {
    pub fn new(graph: Vec<Vec<(usize, usize)>>) -> Self {
        let n: usize = graph.len();
        Self { graph, backptr: vec![INF; n] }
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

                    self.backptr[v] = current_v;
                    queue.push((Reverse(dist[v]), v));
                }
            }
        }

        dist
    }
}


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    let mut edges = HashMap::new();
    let mut edges_id = HashMap::new();

    for i in 0..m {
        let a: usize = scanner.cin::<usize>() - 1;
        let b: usize = scanner.cin::<usize>() - 1;
        let c: usize = scanner.cin();
        graph[a].push((b, c));
        graph[b].push((a, c));

        edges.entry((a, b)).or_insert(c);
        edges.entry((a, b)).and_modify(|e| *e = (*e).min(c));
        edges.entry((b, a)).or_insert(c);
        edges.entry((b, a)).and_modify(|e| *e = (*e).min(c));

        if edges[&(a, b)] == c {
            edges_id.entry((a, b)).or_insert(i);
            edges_id.entry((a, b)).and_modify(|e| *e = i);
            edges_id.entry((b, a)).or_insert(i);
            edges_id.entry((b, a)).and_modify(|e| *e = i);
        }
    }

    let mut dijkstra = Dijkstra::new(graph);
    dijkstra.search(0);

    let mut ans = vec![];
    for i in 1..n {
        ans.push(1 + edges_id[&(i, dijkstra.backptr[i])]);
    }

    let ans = ans
        .iter()
        .map(|&e| e.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", ans);
}


use std::collections::{VecDeque, HashMap};
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
