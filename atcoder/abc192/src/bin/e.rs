use std::collections::BinaryHeap;


const INF: usize = 1_000_000_000_000_000_000;


#[derive(Debug,Clone)]
pub struct Dijkstra {
    graph: Vec<Vec<(usize, usize, usize)>>,
}

impl Dijkstra {
    pub fn new(graph: Vec<Vec<(usize, usize, usize)>>) -> Self {
        Self {
            graph,
        }
    }

    pub fn search(&mut self, src: usize) -> Vec<usize> {
        let mut dist = vec![INF; self.graph.len()];
        dist[src] = 0;

        let mut queue = BinaryHeap::new();
        queue.push((std::cmp::Reverse(0), src));

        while let Some((std::cmp::Reverse(current_cost), current_v)) = queue.pop() {
            if dist[current_v] < current_cost {
                continue;
            }

            for &(v, cost, k) in self.graph[current_v].iter() {
                let next_departure = if current_cost == 0 { 0 } else { k * ((current_cost + k - 1) / k) };
                let wait_duration = if next_departure == 0 { 0 } else { next_departure - current_cost };
                if dist[v] > current_cost + wait_duration + cost {
                    dist[v] = current_cost + wait_duration + cost;
                    queue.push((std::cmp::Reverse(dist[v]), v));
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
    let x: usize = scanner.cin();
    let x = x - 1;
    let y: usize = scanner.cin();
    let y = y - 1;

    let mut g: Vec<Vec<(usize, usize, usize)>> = vec![vec![]; n];

    for _ in 0..m {
        let a: usize = scanner.cin();
        let a = a - 1;
        let b: usize = scanner.cin();
        let b = b - 1;
        let t: usize = scanner.cin();
        let k: usize = scanner.cin();

        g[a].push((b, t, k));
        g[b].push((a, t, k));
    }

    let mut dijkstra = Dijkstra::new(g);
    let dist = dijkstra.search(x);

    let ans = match dist[y] {
        INF => String::from("-1"),
        v => v.to_string(),
    };

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
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
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

#[macro_export]
macro_rules! trace {
    ($x:expr) => {
        #[cfg(debug_assertions)]
        eprintln!(">>> {} = {:?}", stringify!($x), $x)
    };
    ($($xs:expr),*) => { trace!(($($xs),*)) }
}
