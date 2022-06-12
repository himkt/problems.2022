const INF: f64 = 1e30;
struct Point(f64, f64);


fn euclidean(p1: &Point, p2: &Point) -> f64 {
    let v = (p1.0 - p2.0) * (p1.0 - p2.0) + (p1.1 - p2.1) * (p1.1 - p2.1);
    v.sqrt()
}


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();

    // light
    let a: HashSet<usize> = scanner
        .vec(k)
        .iter()
        .map(|ai| ai - 1usize)
        .collect();

    // without light
    let b: HashSet<usize> = (0..n)
        .filter(|idx| !a.contains(idx))
        .collect();

    let mut points = vec![];
    for _ in 0..n {
        let x: f64 = scanner.cin();
        let y: f64 = scanner.cin();
        points.push(Point(x, y));
    }

    let mut ds = vec![];
    for &idx2 in b.iter() {
        let mut minimum = INF;
        for &idx1 in a.iter() {
            let p1 = &points[idx1];
            let p2 = &points[idx2];
            let d = euclidean(p1, p2);
            minimum = minimum.min(d);
        }
        ds.push(minimum);
    }

    let mut ans: f64 = ds[0];
    for d in ds {
        ans = ans.max(d);
    }
    println!("{}", ans);
}


use std::collections::{VecDeque, HashSet};
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
