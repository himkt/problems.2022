fn main() {
    let mut scanner = Scanner::new();
    let l: usize = scanner.cin();
    let q: usize = scanner.cin();

    let mut btree_set: BTreeSet<usize> = BTreeSet::new();
    btree_set.insert(0);
    btree_set.insert(l);

    for _ in 0..q {
        let c: usize = scanner.cin();
        let x: usize = scanner.cin();

        match c {
            1 => {
                btree_set.insert(x);
            },
            2 => {
                let lower_bound = btree_set.range(..x).rev().next().unwrap();
                let upper_bound = btree_set.range(x..).next().unwrap();
                println!("{}", upper_bound - lower_bound);
            },
            _ => panic!(),
        }
    }
}


use std::collections::{VecDeque, BTreeSet};
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
