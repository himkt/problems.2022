fn main() {
    let mut scanner = Scanner::new();
    let s: String = scanner.cin();
    let q: usize = scanner.cin();

    let abc = vec!['A', 'B', 'C'];
    let s: Vec<usize> = s.chars()
        .map(|c| abc.iter().position(|&q| q == c).unwrap())
        .collect();

    for _ in 0..q {
        let t: usize = scanner.cin();
        let k: usize = scanner.cin();
        let k = k - 1;

        let u = k / 2usize.pow(t.min(60) as u32);
        let l = k % 2usize.pow(t.min(60) as u32);
        let popcount: usize = l.count_ones() as usize;

        let v = (s[u] + t + popcount) % 3;
        println!("{}", abc[v]);
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
