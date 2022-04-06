fn main() {
    let mut scanner = Scanner::new();
    let s: String = scanner.cin();
    let s: Vec<char> = s.chars().collect();

    let mut kl: usize = 0;
    for si in s.clone().into_iter() {
        if si != 'a' {
            break;
        }
        kl += 1;
    }

    let mut kr: usize = 0;
    for si in s.clone().into_iter().rev() {
        if si != 'a' {
            break;
        }
        kr += 1;
    }

    if kl > kr {
        println!("No");
        return;
    }

    let l: Vec<char> = vec!['a'; kr - kl];
    let a = [l, s].concat();

    let mut f: bool = true;
    for i in 0..a.len() {
        if a[i] != a[a.len()-i-1] {
            f = false;
        }
    }

    if f {
        println!("Yes");
    }
    else {
        println!("No");
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
