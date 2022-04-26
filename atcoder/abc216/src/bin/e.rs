fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut k: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut heap = BinaryHeap::new();
    for ai in a { heap.push(ai); }

    let mut ans = 0;
    let mut t = 0;

    loop {
        if heap.is_empty() { break; }

        let v = heap.pop().unwrap();
        t += 1;

        while let Some(&nv) = heap.peek() {
            if nv != v {
                break;
            }

            heap.pop();
            t += 1;
        }

        let d = if heap.is_empty() {
            v
        }
        else {
            v - heap.peek().unwrap()
        };

        if k >= t * d {
            let s = d * (2*v - d + 1) / 2;
            ans += t * s;
            k -= t * d;
        }
        else {
            let p = k / t;
            let q = k - p * t;

            ans += t * p * (2*v - p + 1) / 2;
            ans += q * (v - p);
            break;
        }
    }

    println!("{}", ans);
}


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
