const INF: usize = 1_000_000_000_000;

pub fn lower_bound(range: std::ops::Range<usize>, prop: &dyn Fn(usize) -> bool) -> usize {
    if prop(range.start) {
        return range.start;
    }

    let mut ng = range.start;
    let mut ok = range.end;

    while ok - ng > 1 {
        let middle = ng + (ok - ng) / 2;
        match prop(middle) {
            true => ok = middle,
            false => ng = middle,
        }
    }

    ok
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let mut points = vec![];
    let mut powers = vec![];
    for _ in 0..n {
        let x: i128 = scanner.cin();
        let y: i128 = scanner.cin();
        let p: usize = scanner.cin();
        points.push((x, y));
        powers.push(p);
    }

    let mut dist = vec![vec![INF; n]; n];
    for i in 0..n {
        dist[i][i] = 0;

        for j in 0..n {
            let dx = (points[i].0 - points[j].0).abs();
            let dy = (points[i].1 - points[j].1).abs();
            dist[i][j] = (dx + dy) as usize;
        }
    }

    let construct = |s: usize| -> Vec<Vec<usize>> {
        let mut graph = vec![vec![]; n];

        for i in 0..n {
            for j in 0..n {
                if powers[i] * s >= dist[i][j] {
                    graph[i].push(j);
                }
            }
        }

        graph
    };

    let connected = |s: usize| -> bool {
        let graph = construct(s);

        for root in 0..n {
            let mut queue = VecDeque::new();
            queue.push_back(root);

            let mut used = vec![false; n];
            while let Some(u) = queue.pop_front() {
                used[u] = true;

                for i in 0..graph[u].len() {
                    let v = graph[u][i];
                    if used[v] { continue; }
                    queue.push_back(v);
                }
            }

            if used.iter().all(|&x| x) {
                return true;
            }
        }

        false
    };

    let ans = lower_bound(0..INF, &|x| connected(x));
    println!("{}", ans);
}

use std::{io::Write, collections::VecDeque};
pub fn flush() { std::io::stdout().flush().unwrap(); }
pub struct Scanner { stdin: std::io::Stdin, buffer: std::collections::VecDeque<String> }
impl Scanner {
    fn new() -> Self {
        Self { stdin: std::io::stdin(), buffer: std::collections::VecDeque::new() }
    }

    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            line.split_whitespace().for_each(|x| self.buffer.push_back(String::from(x)));
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}
