#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut heap = BinaryHeap::new();
    for &ai in a.iter() {
        if ai > 0 {
            heap.push(Reverse(ai));
        }
    }

    let mut tot_iters = 0;
    let mut tot_apples = 0;

    while let Some(Reverse(v)) = heap.pop() {
        let alen = heap.len() + 1;
        let cur_iters = ((k - tot_apples + alen - 1) / alen).min(v - tot_iters);

        tot_apples += cur_iters * alen;
        tot_iters += cur_iters;

        if tot_apples >= k {
            break;
        }
    }

    if cfg!(debug_assertions) {
        println!("tot_apples={}", tot_apples);
        println!("tot_iters={}", tot_iters);
    }

    let mut b = vec![0; n];
    for i in 0..n {
        b[i] = a[i].min(tot_iters - 1);
    }

    if cfg!(debug_assertions) {
        dbg!(&b);
    }

    let mut btot: usize = b.iter().sum();
    for i in 0..n {
        if btot == k {
            break;
        }
        if a[i] > b[i] {
            b[i] += 1;
            btot += 1;
        }
    }

    if cfg!(debug_assertions) {
        dbg!(&a);
        dbg!(&b);
    }

    for i in 0..n {
        println!("{}", a[i] - b[i]);
    }
}

use std::{io::Write, collections::BinaryHeap, cmp::Reverse};
pub struct Scanner {
    buffer: std::collections::VecDeque<String>,
    buf: String,
}
#[allow(clippy::new_without_default)]
impl Scanner {
    pub fn new() -> Self {
        Scanner {
            buffer: std::collections::VecDeque::new(),
            buf: String::new(),
        }
    }
    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        if !self.buffer.is_empty() {
            return self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap();
        }
        self.buf.truncate(0);
        std::io::stdin().read_line(&mut self.buf).ok();
        self.buf
            .to_owned()
            .split_whitespace()
            .for_each(|x| self.buffer.push_back(String::from(x)));
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
    pub fn flush(&self) {
        std::io::stdout().flush().unwrap();
    }
}
