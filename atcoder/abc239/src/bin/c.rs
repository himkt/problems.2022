fn main() {
    let mut scanner = Scanner::new();
    let x1: i64 = scanner.cin();
    let y1: i64 = scanner.cin();
    let x2: i64 = scanner.cin();
    let y2: i64 = scanner.cin();

    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();

    let ds = vec![(1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1), (-1, 2)];
    for (dx, dy) in ds {
        set1.insert((x1 + dx, y1 + dy));
        set2.insert((x2 + dx, y2 + dy));
    }

    let intersection: Vec<_> = set1.intersection(&set2).collect();
    if intersection.is_empty() {
        println!("No");
    }
    else {
        println!("Yes");
    }
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
