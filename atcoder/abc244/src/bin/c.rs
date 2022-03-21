fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let mut used = vec![false; 2*n+1];
    loop {
        let iter = used
            .iter()
            .enumerate()
            .filter(|&(_, &u)| !u)
            .map(|(idx, &_)| idx + 1);

        let candidates: Vec<_> = iter.collect();
        if candidates.is_empty() {
            break;
        }

        println!("{}", candidates[0]);
        used[candidates[0]-1] = true;

        let input: usize = scanner.cin();
        flush();

        if input == 0 {
            break;
        }
        used[input-1] = true;
    }
}


use std::collections::VecDeque;
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
