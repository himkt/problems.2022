fn main() {
    let mut scanner = Scanner::new();
    let q: usize = scanner.cin();
    let mut priority_queue = BinaryHeap::new();
    let mut deque = VecDeque::new();

    for _ in 0..q {
        let qi: usize = scanner.cin();

        if qi == 1 {
            let x: usize = scanner.cin();
            deque.push_back(x);
        }
        else if qi == 2 {
            if !priority_queue.is_empty() {
                let Reverse(v) = priority_queue.pop().unwrap();
                println!("{}", v);
            }
            else {
                let v = deque.pop_front().unwrap();
                println!("{}", v);
            }
        }
        else if qi == 3 {
            while let Some(v) = deque.pop_front() {
                priority_queue.push(Reverse(v));
            }
        }
    }
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
