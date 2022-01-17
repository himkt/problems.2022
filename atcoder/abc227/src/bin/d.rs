pub trait BinarySearch<T> {
    fn lower_bound(&self, k: usize, p: usize) -> usize;
}


impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, k: usize, p: usize) -> usize {
        let mut left  = 0;
        let mut right = self.len();

        // update here
        let check = |mid| {
            k * mid <= p
        };

        while left < right {
            let mid = left + (right - left) / 2;

            if check(mid) {
                right = mid;
            }
            else {
                left = mid + 1;
            }

        }

        left
    }
}


#[allow(clippy::many_single_char_names)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut l: usize = 0;
    let mut r: usize = n + 1;

    while l < r {
        let num_projects = l + (r - l) / 2;

        let mut capacity: usize = 0;
        for &ai in &a {
            capacity += ai.min(num_projects);
        }

        if capacity >= num_projects * k {
            r = num_projects;
        }
        else {
            l = num_projects + 1;
        }
    }

    println!("{}", l);
}


use std::io::{self, Write};
use std::str::FromStr;
use std::collections::VecDeque;

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
