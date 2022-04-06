const DIV: usize = 1e9 as usize + 7;
const INF: usize = 1e9 as usize + 7;


fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..m {
        let a: usize = scanner.cin();
        let b: usize = scanner.cin();
        graph[a-1].push(b-1);
        graph[b-1].push(a-1);
    }

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((0, 0));

    let mut used = vec![false; n];

    let mut dist = vec![INF; n];
    dist[0] = 0;

    let mut dp = vec![0; n];
    dp[0] = 1;

    while let Some((head, cost)) = queue.pop_front() {
        if used[head] {
            continue;
        }

        used[head] = true;

        for &next in &graph[head] {
            if used[next] {
                continue;
            }

            let new_cost = cost + 1;
            queue.push_back((next, new_cost));

            match new_cost.cmp(&dist[next]) {
                Ordering::Equal => {
                    dp[next] += dp[head];
                    dp[next] %= DIV;
                },
                Ordering::Less => {
                    dist[next] = new_cost;
                    dp[next] = dp[head];
                    dp[next] %= DIV;
                },
                Ordering::Greater => {},
            }
        }
    }

    let ans = dp.last().unwrap();
    println!("{}", ans);
}


use std::cmp::Ordering;
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

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

#[allow(dead_code)]
fn flush() {
    std::io::stdout().flush().unwrap();
}
