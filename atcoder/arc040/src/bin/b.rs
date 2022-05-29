#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let r: usize = scanner.cin();

    let mut cs: Vec<char> = scanner.cin::<String>().chars().collect();
    let mut record_time = 0;
    let mut current_time = 0;
    let mut i = 0;

    let last_idx_option = cs
        .iter()
        .enumerate()
        .filter(|&(_, c)| c == &'.')
        .map(|(i, _)| i)
        .last();

    while i < n {
        let max_target_idx = (i + r - 1).min(n-1);

        if let Some(last_idx) = last_idx_option {
            if last_idx <= max_target_idx {
                current_time += 1;
                record_time = current_time;
                break;
            }
        }

        if cs[i] == 'o' {
            current_time += 1;
            i += 1;
        }
        else {
            for diff in 0..r {
                if i + diff == n {
                    break;
                }
                cs[i + diff] = 'o';
            }
            current_time += 1;
            record_time = current_time;
        }
    }

    println!("{}", record_time);
}


use std::collections::VecDeque;
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
