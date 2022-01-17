#[allow(clippy::branches_sharing_code)]
fn main() {
    let mut scanner = Scanner::new();
    let s: String = scanner.cin();
    let s: Vec<char> = s.chars().collect();
    let k: usize = scanner.cin();
    let n: usize = s.len();

    let mut cumnum: Vec<usize> = vec![0; n+1];
    for i in 1..=n {
        if s[i-1] == '.' {
            cumnum[i] = cumnum[i-1] + 1;
        }
        else {
            cumnum[i] = cumnum[i-1];
        }
    }

    let mut ans: usize = 0;
    let mut r: usize = 1;
    for l in 1..=n {
        while r < cumnum.len() && cumnum[r] <= cumnum[l-1] + k {
            r += 1;
        }

        ans = ans.max(r - l);
    }

    println!("{}", ans);
}


use std::io::{self, Write};
use std::str::FromStr;
use std::collections::{VecDeque};

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
