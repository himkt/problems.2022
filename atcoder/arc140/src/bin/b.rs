#[allow(clippy::needless_range_loop)]
#[allow(clippy::collapsible_else_if)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let s: String = scanner.cin();
    let s: Vec<char> = s.chars().collect();

    let mut queue_even = VecDeque::new();
    let mut queue_odd = VecDeque::new();

    for i in 1..s.len()-1 {
        if s[i-1] == 'A' && s[i] == 'R' && s[i+1] == 'C' {
            if i - 1 == 0 || i + 1 == n - 1 {
                queue_odd.push_back((i-1, i+1));
            }
            else if s[i - 2] == 'A' && s[i + 2] == 'C' {
                queue_even.push_back((i-1, i+1));
            }
            else {
                queue_odd.push_back((i-1, i+1));
            }
        }
    }

    let mut ans = 0;
    for i in 1..=n {
        if queue_even.is_empty() && queue_odd.is_empty() { break; }

        let selected_queue_index = if i % 2 == 1 {
            if !queue_even.is_empty() {
                0  // even
            }
            else {
                1  // odd
            }
        }
        else {
            if !queue_odd.is_empty() {
                1  // odd
            }
            else {
                0  // even
            }
        };

        let head = if selected_queue_index == 0 {
            queue_even.pop_front().unwrap()
        }
        else {
            queue_odd.pop_front().unwrap()
        };

        ans += 1;

        if head.0 == 0 || head.1 == n - 1 {
            continue;
        }

        if i % 2 == 1 && s[head.0 - 1] == 'A' && s[head.1 + 1] == 'C' {
            if head.0 - 1 == 0 || head.1 + 1 == n - 1 {
                queue_odd.push_back((head.0 - 1, head.1 + 1));
            }
            else if s[head.0 - 2] == 'A' && s[head.1 + 2] == 'C' {
                queue_even.push_back((head.0 - 1, head.1 + 1));
            }
            else {
                queue_odd.push_back((head.0 - 1, head.1 + 1));
            }
        }
    }

    println!("{}", ans);
}


use std::collections::{VecDeque};
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
