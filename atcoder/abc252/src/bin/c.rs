#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let slots: Vec<Vec<char>> = (0..n)
        .map(|_| {
            let s: String = scanner.cin();
            let cs: Vec<char> = s.chars().collect();
            cs
        })
        .collect();

    let numbers = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    // println!("slots: {:?}", slots);

    let mut ans = 1001001001001;

    for &target in numbers.iter() {
        let mut t = 0;
        let mut used = vec![false; n];

        loop {
            for i in 0..n {
                if !used[i] && slots[i][t%10] == target {
                    used[i] = true;
                    break;
                }
            }

            // println!("t={}, target={}, used={:?}", t, target, used);

            let mut finished = true;
            for i in 0..n {
                if !used[i] {
                    finished = false;
                }
            }

            if !finished {
                t += 1;
                continue;
            }

            break;
        }

        // println!("t={}, target={}", t, target);
        ans = ans.min(t);
    }

    println!("{}", ans);
}


use std::collections::{VecDeque, HashMap};
use std::io::{self, Write};
use std::str::FromStr;

pub struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
impl Scanner {
    fn new() -> Self {
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
    }

    pub fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

pub fn flush() {
    std::io::stdout().flush().unwrap();
}
