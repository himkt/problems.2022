fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let q: usize = scanner.cin();

    let mut nexts: Vec<Option<usize>> = vec![None; n+2];
    let mut prevs: Vec<Option<usize>> = vec![None; n+2];

    for _ in 0..q {
        let t: usize = scanner.cin();

        if t == 1 {
            let x: usize = scanner.cin();
            let y: usize = scanner.cin();
            nexts[x] = Some(y);
            prevs[y] = Some(x);
        }
        else if t == 2 {
            let x: usize = scanner.cin();
            let y: usize = scanner.cin();
            nexts[x] = None;
            prevs[y] = None;
        }
        else if t == 3 {
            let mut x: usize = scanner.cin();
            let mut vs: Vec<usize> = vec![x];

            // backward
            while let Some(v) = prevs[x] {
                x = v;
            }

            // forward
            while let Some(v) = nexts[x] {
                vs.push(v);
                x = v;
            }

            let vs: Vec<String> = vs.iter().map(|&x| x.to_string()).collect();
            println!("{} {}", vs.len(), vs.join(" "));
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
