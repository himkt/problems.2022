fn main() {
    let mut scanner = Scanner::new();
    let t: usize = scanner.cin();

    for _ in 0..t {
        let a: usize = scanner.cin();
        let s: usize = scanner.cin();

        let mut d1 = 0;
        let mut d2 = 0;
        let mut carry = false;

        for i in 0..62 {
            let k1 = (a >> i) & 1;
            let k2 = (s >> i) & 1;

            if k1 == 1 {
                d1 |= 1 << i;
                d2 |= 1 << i;
                carry = true;
            } else if carry {
                if k2 == 1 {
                    carry = false;
                } else if k2 == 0 {
                    d1 |= 1 << i;
                    carry = true;
                }
            } else {
                if k2 == 1 {
                    d1 |= 1 << i;
                    carry = false;
                } else if k2 == 0 {
                    carry = false;
                }
            }
        }

        if d1 + d2 == s && d1 & d2 == a {
            println!("Yes");
        } else {
            println!("No");
        }
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
