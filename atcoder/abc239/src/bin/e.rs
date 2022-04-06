struct Tree {
    g: Vec<Vec<usize>>,
    xs: Vec<usize>,
    children: Vec<Vec<usize>>,
}

impl Tree {
    fn new(g: Vec<Vec<usize>>, xs: Vec<usize>) -> Self {
        let n = xs.len();
        Tree { g: g, xs: xs, children: vec![vec![]; n] }
    }

    fn construct(&mut self, node_id: usize) -> Vec<usize> {

    }
}




fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let q: usize = scanner.cin();

    let xs: Vec<usize> = scanner.vec(n);

    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..(n-1) {
        let a: usize = scanner.cin();
        let b: usize = scanner.cin();
        g[a-1].push(b-1);
        g[b-1].push(a-1);
    }

    for _ in 0..q {
        let v: usize = scanner.cin();
        let k: usize = scanner.cin();
    }

    println!("{:?}", g);
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
