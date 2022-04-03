fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut k: usize = scanner.cin();
    let x: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut heap = BinaryHeap::new();
    for ai in a {
        heap.push(ai);
    }

    while let Some(v) = heap.pop() {
        if k == 0 {
            heap.push(v);
            break;
        }

        if v >= x {
            let num_coupons = (v / x).min(k);
            let discount = num_coupons * x;

            if discount < v {
                heap.push(v - discount);
            }

            k -= num_coupons;
        }
        else {
            k -= 1;
        }
    }

    let mut ans = 0;
    for v in heap.iter() {
        ans += v;
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
