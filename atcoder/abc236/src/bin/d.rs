struct Calculator {
    ans: usize,
}

impl Calculator {
    fn new() -> Self {
        Calculator { ans: 0 }
    }

    fn find(&mut self, s: &mut Vec<bool>, a: &Vec<Vec<usize>>, x: usize) {
        if let Some(si) = (0..s.len()).filter(|&i| !s[i]).next() {
            s[si] = true;
            for i in 0..s.len() {
                if !s[i] && si != i {
                    // (a) 使う場合の再帰
                    s[i] = true;
                    self.find(&mut s.clone(), a, x^a[si][i]);

                    // (b) i を使わない場合の後続のループのため
                    s[i] = false;
                }
            }
        } else {
            self.ans = self.ans.max(x);
            return;
        }
    }
}


fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut a: Vec<Vec<usize>> = vec![vec![0; 2*n]; 2*n];
    let mut s: Vec<bool> = vec![false; 2*n];

    for i in 0..2*n {
        for j in i+1..2*n {
            let aij: usize = scanner.cin();
            a[i][j] = aij;
            a[j][i] = aij;
        }
    }

    let mut calc = Calculator::new();
    calc.find(&mut s, &a, 0);
    println!("{}", calc.ans);
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
