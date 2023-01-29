#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();
    let mut chbox: HashSet<char> = HashSet::new();
    let mut stack: VecDeque<char> = VecDeque::new();

    for si in s {
        match si {
            '(' => stack.push_front(si),
            ')' => {
                while let Some(head) = stack.pop_front() {
                    if head == '(' {
                        break;
                    }
                    chbox.remove(&head);
                }
            }
            _ => {
                if chbox.contains(&si) {
                    println!("No");
                    return;
                }
                chbox.insert(si);
                stack.push_front(si);
            }
        }
    }

    println!("Yes");
}

use std::{io::Write, collections::{HashSet, VecDeque}};
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

#[macro_export]
macro_rules! debug {
    () => {
        #[cfg(debug_assertions)]
        println!();
    };
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}
