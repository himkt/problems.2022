fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let s: String = scanner.cin();

    let mut heap = BinaryHeap::new();
    for (i, c) in s.chars().enumerate() {
        heap.push((Reverse(c), i));
    }

    let mut r = n;
    let mut ans: Vec<char> = s.chars().collect();

    for (i, ci) in s.chars().enumerate() {

        while !heap.is_empty() {
            let &(Reverse(cj), j) = heap.peek().unwrap();

            if ci <= cj { break; }
            heap.pop();

            if i >= j { continue; }

            if cj < ci && j < r {
                ans.swap(i, j);
                r = j;
                break;
            }
        }
    }

    let ans: String = ans.iter().collect();
    println!("{}", ans);
}


use std::cmp::Reverse;
use std::collections::{VecDeque, BinaryHeap};
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
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
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
