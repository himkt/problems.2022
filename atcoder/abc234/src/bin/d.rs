#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();
    let mut p: Vec<usize> = scanner.vec(n);

    let mut heap = BinaryHeap::new();
    p[0..k].sort_unstable();

    for i in 0..k {
        heap.push(Reverse(p[i]));
    }

    println!("{}", heap.peek().unwrap().0);

    for i in k..n {
        let Reverse(min) = heap.peek().unwrap();

        if min > &p[i] {
            println!("{}", min);
        }
        else {
            heap.pop();
            heap.push(Reverse(p[i]));
            let Reverse(pi) = heap.peek().unwrap();
            println!("{}", pi);
        }
    }
}


use std::cmp::Reverse;
use std::io::{self, Write};
use std::str::FromStr;
use std::collections::{VecDeque, BinaryHeap};

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
