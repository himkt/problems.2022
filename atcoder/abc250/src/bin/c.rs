#[allow(clippy::needless_range_loop)]
#[allow(clippy::manual_swap)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let q: usize = scanner.cin();

    let mut positions: HashMap<usize, usize> = HashMap::new();
    for i in 1..=n {
        positions.insert(i, i);
    }

    let mut a: Vec<usize> = (0..=n).collect();
    for _ in 0..q {
        let x: usize = scanner.cin();
        let px = positions[&x];
        let py = match px == n {
            true => px - 1,
            false => px + 1,
        };

        let tmp = a[px];
        a[px] = a[py];
        a[py] = tmp;

        positions.entry(a[px]).and_modify(|e| *e = px);
        positions.entry(a[py]).and_modify(|e| *e = py);
    }

    let mut ans = vec![];
    for i in 1..=n {
        let v = a[i].to_string();
        ans.push(v);
    }

    println!("{}", ans.join(" "));
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
