#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();

    let mut ps: Vec<Vec<char>> = vec![vec!['.'; w]; h];

    for i in 0..h {
        let s: String = scanner.cin();
        let cs: Vec<char> = s.chars().collect();
        ps[i] = cs;
    }

    let mut ans = 0;
    for i in 0..h-1 {
        for j in 0..w-1 {
            let mut cnt = 0;
            if ps[i][j] == '#' { cnt += 1; }
            if ps[i+1][j] == '#' { cnt += 1; }
            if ps[i][j+1] == '#' { cnt += 1; }
            if ps[i+1][j+1] == '#' { cnt += 1; }
            if cnt == 1 || cnt == 3 {
                ans += 1;
            }
        }
    }

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
