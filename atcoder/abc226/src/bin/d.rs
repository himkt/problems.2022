pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut pairs: Vec<(i64, i64)> = vec![];
    for _ in 0..n {
        let raw: Vec<i64> = scanner.vec(2);
        let pair: (i64, i64) = (raw[0], raw[1]);
        pairs.push(pair);
    }

    pairs.sort_unstable();
    let mut paths: HashSet<(i64, i64)> = HashSet::new();

    for i in 0..n {
        for j in i+1..n {
            let pi = pairs[i];
            let pj = pairs[j];

            let path = (pj.0 - pi.0, pj.1 - pi.1);
            let gcd = gcd(path.0, path.1);
            let path_shrinked = (path.0 / gcd, path.1 / gcd);
            paths.insert(path_shrinked);
        }
    }

    let ans = 2 * paths.len();
    println!("{}", ans);
}


use std::collections::{VecDeque, HashSet};
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
