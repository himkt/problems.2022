#[derive(PartialEq, Eq, Debug)]
pub enum State {
    O,
    A,
    R,
    C,
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let _: usize = scanner.cin();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();

    let mut num_a = 0;
    let mut num_c = 0;
    let mut prev_state = State::O;

    // construction
    let mut vs = vec![];
    for ci in s {
        match ci {
            'A' => {
                if prev_state == State::A {
                    num_a += 1;
                    num_c = 0;
                }
                else {
                    let v = num_a.min(num_c);
                    vs.push(v);
                    num_a = 1;
                    num_c = 0;
                }
            },
            'R' => {
                if prev_state == State::A {
                    num_c = 0;
                }
                else {
                    let v = num_a.min(num_c);
                    vs.push(v);
                    num_a = 0;
                    num_c = 0;
                }
            },
            'C' => {
                if prev_state == State::R {
                    num_c = 1;
                }
                else if prev_state == State::C {
                    num_c += 1;
                }
                else {
                    let v = num_a.min(num_c);
                    vs.push(v);
                    num_a = 0;
                    num_c = 0;
                }
            },
            _   => { panic!() },
        }

        prev_state = match ci {
            'A' => State::A,
            'R' => State::R,
            'C' => State::C,
            _   => panic!(),
        }
    }

    let v = num_a.min(num_c);
    vs.push(v);

    // calculation
    let mut ans = 0;
    let mut num_singular = 0;
    let mut deque = VecDeque::new();

    vs.sort_unstable();
    for v in vs {
        if v == 1 {
            num_singular += 1;
        }
        else if v > 1 {
            deque.push_back(v);
        }
    }

    for i in 1..=200_000 {
        if i % 2 == 1 {
            if !deque.is_empty() {
                deque[0] -= 1;
                if deque[0] == 1 {
                    deque.pop_front();
                    num_singular += 1;
                }
                ans += 1;
            }
            else if num_singular > 0 {
                num_singular -= 1;
                ans += 1;
            }
            else {
                break;
            }
        }
        else {
            if num_singular > 0 {
                num_singular -= 1;
                ans += 1;
            }
            else if !deque.is_empty() {
                deque[0] -= 1;
                deque.pop_front();
                ans += 1;
            }
            else {
                break;
            }
        }
    }

    println!("{}", ans);
}

use std::{io::Write, collections::VecDeque};
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

