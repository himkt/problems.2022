pub trait BinarySearch<T> {
    fn lower_bound(&self, query: T) -> usize;
}


impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, query: T) -> usize {
        let mut left  = 0;
        let mut right = self.len();

        // update here
        let check = |mid| {
            self[mid] >= query
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


fn main() {
    let mut scanner = Scanner::new();
    let n: i64 = scanner.cin();

    // a_i = a*x + b
    // -9 <= a <= 9
    // 0  <= b <= 9

    let mut xs = HashSet::new();

    for a in -9..=9 {
        for b in -20..=20 {

            let mut p = vec![];
            for x in 1..=18 {
                let ai = a*x + b;
                if ai >= 10 || ai <= -1 {
                    break;
                }
                p.push(ai);

                let ps: Vec<_> = p.iter().map(|&x| x.to_string()).collect();
                if !ps.is_empty() {
                    let num: i64 = ps.join("").parse().unwrap();
                    if num > 0 {
                        xs.insert(num);
                    }

                }
            }
        }
    }

    let mut xs: Vec<_> = xs.iter().collect();
    xs.sort();

    let i = xs.lower_bound(&n);
    println!("{}", xs[i]);
}


use std::io::{self, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashSet};

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
