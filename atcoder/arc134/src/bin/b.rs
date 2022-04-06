fn main() {
    let mut scanner = Scanner::new();
    let _: usize = scanner.cin();
    let s: String = scanner.cin();
    let mut s: Vec<char> = s.chars().collect();

    let mut l = 0;
    let mut r = s.len() - 1;

    while l < r {
        let mut c = s[l];
        let mut update = false;

        for i in ((l+1)..=r).rev() {
            if s[i] < c {
                update = true;
                r = i;
                c = s[i];
            }
        }

        if !update {
            l += 1;
            continue;
        }

        // swap
        let tmp = s[l];
        s[l] = s[r];
        s[r] = tmp;

        l += 1;
        r -= 1;
    }

    let ans: String = s.iter().collect();
    println!("{}", ans);
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
