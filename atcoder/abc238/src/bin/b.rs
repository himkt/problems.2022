fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut na: Vec<usize> = vec![0; n];
    na[0] = a[0];

    for i in 1..n {
        na[i] = (na[i-1] + a[i]) % 360;
    }

    na.sort();

    let mut nna = vec![0; n+2];
    nna[n+1] = 360;

    for i in 0..n {
        nna[i+1] = na[i];
    }

    let mut ans = 0;

    for i in 1..(n+2) {
        // println!("{} - {}", nna[i], nna[i-1]);
        ans = ans.max(nna[i] - nna[i-1]);
    }

    // println!("{:?}", nna);
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
