pub fn prime_sieve(n: usize) -> Vec<bool> {
    let mut s = vec![true; n];
    s[0] = false;
    s[1] = false;
    for i in 2..n {
        if i * i > n {
            break;
        }
        if s[i] {
            for k in 2..(n + i - 1) / i {
                s[k * i] = false
            }
        }
    }
    s
}


fn main() {
    let mut scanner = Scanner::new();
    let a: usize = scanner.cin();
    let b: usize = scanner.cin();
    let c: usize = scanner.cin();
    let d: usize = scanner.cin();

    let ps = prime_sieve(205);

    let mut wins = HashSet::new();

    for k in a..=b {
        let mut winner = "Takahashi";

        for p in c..=d {
            let sum = k + p;

            if ps[sum] {
                winner = "Aoki";
                break;
            }
        }
        wins.insert(winner);
    }

    if wins.contains(&"Takahashi") {
        println!("Takahashi");
    }
    else {
        println!("Aoki");
    }
}


use std::collections::{VecDeque, HashSet};
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
