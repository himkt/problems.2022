#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut c: VecDeque<(usize, usize)> = VecDeque::new();

    let mut cnt = 0;
    c.push_front((a[0], 1));
    cnt += 1;
    println!("{}", cnt);
    // println!("{:?}", c);

    for i in 1..n {
        if let Some(head) = c.front() {
            if head.0 == a[i] {
                if a[i] == head.1 + 1 {
                    cnt -= head.1;
                    c.pop_front();
                }
                else {
                    cnt += 1;
                    (*c.front_mut().unwrap()).1 += 1;
                }
            }
            else {
                cnt += 1;
                c.push_front((a[i], 1));
            }
        }
        else {
            cnt += 1;
            c.push_front((a[i], 1));
        }

        println!("{}", cnt);
        // println!("{:?}", c);
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

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

#[allow(dead_code)]
fn flush() {
    std::io::stdout().flush().unwrap();
}
