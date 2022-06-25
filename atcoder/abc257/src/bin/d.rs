const INF: usize = 1_000_000_000_000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let mut points = vec![];
    for _ in 0..n {
        let x: i128 = scanner.cin();
        let y: i128 = scanner.cin();
        points.push((x, y));
    }

    let mut graph = vec![vec![INF; n]; n];
    for i in 0..n {
        for j in 0..n {
            let dx = (points[i].0 - points[j].0).abs();
            let dy = (points[i].1 - points[j].1).abs();
            graph[i][j] = (dx + dy) as usize;
        }
    }

    println!("{:?}", graph);

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                graph[i][j] = graph[i][j].min(graph[i][k] + graph[k][j]);
            }
        }
    }

    println!("{:?}", graph);
}

use std::io::Write;
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
