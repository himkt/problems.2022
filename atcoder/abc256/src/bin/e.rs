struct Solver {
    used: Vec<usize>,
    x: Vec<usize>,
    c: Vec<usize>,
}

impl Solver {
    fn dfs(&mut self, u: usize, t: usize) -> usize {
        self.used[u] = t;
        let v = self.x[u];

        if self.used[v] == t {
            let mut tmp = self.x[u];
            let mut cost = self.c[u];

            while tmp != u {
                cost = cost.min(self.c[tmp]);
                tmp = self.x[tmp];
            }
            return cost;
        }

        if self.used[v] != 0 {
            return 0;
        }

        self.dfs(v, t)
    }
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let x: Vec<usize> = scanner.vec::<usize>(n).into_iter().map(|v| v - 1).collect();
    let c: Vec<usize> = scanner.vec(n);
    let used = vec![0; n];

    let mut ans = 0;
    let mut solver = Solver { used, x, c };
    for u in 0..n {
        if solver.used[u] != 0 {
            continue;
        }
        ans += solver.dfs(u, u + 1);
    }

    println!("{}", ans);
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
