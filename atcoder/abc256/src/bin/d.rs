#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let mut lrs: Vec<(usize, usize)> = (0..n)
        .map(|_| {
            let l: usize = scanner.cin();
            let r: usize = scanner.cin();
            (l, r)
        })
        .collect();

    lrs.sort_unstable();

    let mut ans = vec![];
    let mut cur = 0;
    ans.push(lrs[0]);

    for i in 0..n {
        if lrs[i].0 <= ans[cur].1 {
            ans[cur].1 = ans[cur].1.max(lrs[i].1);
        }
        else {
            ans.push(lrs[i]);
            cur += 1;
        }
    }

    for (l, r) in ans {
        println!("{} {}", l, r);
    }
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
