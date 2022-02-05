const DIV: u128 = 998244353;


fn main() {
    let mut scanner = Scanner::new();
    let n: u128 = scanner.cin();
    let mut ans: u128 = 0;

    for i in 1..19 {
        let left: u128 = 10u128.pow(i-1);
        let mut right: u128 = 10u128.pow(i) - 1;
        let mut stop = false;

        if n <= right {
            right = n;
            stop = true;
        }

        let k = right - left + 1;
        ans += k*(k+1) / 2;

        if stop {
            break;
        }
    }

    ans %= DIV;
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
