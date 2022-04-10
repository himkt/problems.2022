fn access(a: &[usize], index: usize, flip: bool) -> usize {
    let ai = a[index];
    if flip {
        if ai == 0 {
            1
        }
        else if ai == 1 {
            0
        }
        else {
            panic!();
        }
    }
    else {
        ai
    }
}

fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut head = 0;
    let mut tail = n-1;
    let mut flip = false;

    while head < tail {
        if access(&a, tail, flip) == 0 {
            tail -= 1;
        }
        else if access(&a, head, flip) == 0 {
            head += 1;
            flip = !flip;
        }
        else {
            println!("No");
            return;
        }
    }

    if access(&a, head, flip) == 0 {
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
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
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

#[macro_export]
macro_rules! trace {
    ($x:expr) => {
        #[cfg(debug_assertions)]
        eprintln!(">>> {} = {:?}", stringify!($x), $x)
    };
    ($($xs:expr),*) => { trace!(($($xs),*)) }
}
