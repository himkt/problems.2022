fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let mut xys = vec![];
    for _ in 0..n {
        let xy: Vec<f64> = scanner.vec(2);
        xys.push(xy);
    }

    let mut ans: f64 = 0.0;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let xd = (xys[j][0] - xys[i][0]).abs();
            let yd = (xys[j][1] - xys[i][1]).abs();
            ans = ans.max((xd*xd + yd*yd).sqrt());
        }
    }

    println!("{}", ans);
}


use std::io::{self, Write};
use std::str::FromStr;
use std::collections::VecDeque;

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
