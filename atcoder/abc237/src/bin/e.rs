use std::collections::BinaryHeap;


const INF: i64 = 1001001001001001;


#[derive(Debug,Clone)]
pub struct Dijkstra {
    graph: Vec<Vec<(usize, i64)>>,
}

impl Dijkstra {
    pub fn new(graph: Vec<Vec<(usize, i64)>>) -> Self {
        Self {
            graph,
        }
    }

    pub fn search(&mut self, src: usize) -> Vec<i64> {
        let mut dist = vec![INF; self.graph.len()];
        dist[src] = 0;

        let mut used = vec![false; self.graph.len()];

        let mut queue = BinaryHeap::new();
        queue.push((std::cmp::Reverse(0), src));

        while let Some((current_cost, current_v)) = queue.pop() {
            if dist[current_v] < current_cost.0 {
                continue;
            }

            if used[current_v] {
                continue;
            }
            // used [current_v] = true;

            for (v, cost) in self.graph[current_v].clone() {
                if dist[v] > current_cost.0 + cost {
                    dist[v] = current_cost.0 + cost;
                    queue.push((std::cmp::Reverse(dist[v]), v));
                }
            }
        }

        dist
    }
}


fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let h: Vec<i64> = scanner.vec(n);

    let mut g: Vec<Vec<(usize, i64)>> = vec![vec![]; n];

    for _ in 0..m {
        let u: usize = scanner.usize1();
        let v: usize = scanner.usize1();

        let d = h[u] - h[v];

        if h[u] > h[v] {
            g[u].push((v, d));
            g[v].push((u, -2*d));
            // g[u][v] = Some(d);
            // g[v][u] = Some(-2*d);
        }
        else if h[u] < h[v] {
            g[u].push((v, 2*d));
            g[v].push((u, -d));
            // g[u][v] = Some(2*d);
            // g[v][u] = Some(d);
        }
        else {
            g[u].push((v, 0));
            g[v].push((u, 0));
            // g[u][v] = Some(0);
            // g[v][u] = Some(0);
        }
    }

    let mut search = Dijkstra::new(g);
    let dist = search.search(0);
    // println!("{:?}", dist);
    let ans = 0.max(*dist.iter().max().unwrap());
    println!("{}", ans);
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
