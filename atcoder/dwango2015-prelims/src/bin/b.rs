#[derive(Debug, PartialEq, Eq)]
enum State { Two, Fiv, Non }


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let cs: Vec<char> = scanner.cin::<String>().chars().collect();
    let n: usize = cs.len();

    let mut num_nico = 0;
    let mut state = State::Non;
    let mut ans = 0;

    for i in 0..n {
        if cs[i] == '2' {
            match state {
                State::Fiv => {},
                _ => num_nico = 0,
            };
            state = State::Two;
        }
        else if cs[i] == '5' {
            match state {
                State::Two => num_nico += 1,
                _ => num_nico = 0,
            };
            ans += num_nico;
            state = State::Fiv;
        }
        else {
            num_nico = 0;
            state = State::Non;
        }
    }

    println!("{}", ans);
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
