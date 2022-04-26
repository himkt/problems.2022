#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let q: usize = scanner.cin();

    let edges: Vec<(usize, usize, usize, usize)> = (0..q)
        .map(|_| {
            let a: usize = scanner.cin();
            let b: usize = scanner.cin();
            let c: usize = scanner.cin();
            let d: usize = scanner.cin();
            (a-1, b-1, c, d)
        })
        .collect();


    let mut ans: usize = 0;

    let mut stack: VecDeque<Vec<usize>> = VecDeque::new();
    stack.push_front(vec![]);

    while let Some(head) = stack.pop_front() {
        if head.len() == n {
            let score = edges.iter()
                .filter(|&&(a, b, c, _)| head[b] - head[a] == c)
                .map(|(_, _, _, d)| d)
                .sum();

            ans = ans.max(score);
            continue;
        }

        let minimum = match head.len() {
            0 => 1,
            v => head[v-1],
        };

        for i in minimum..=m {
            let mut nhead = head.clone();
            nhead.push(i);
            stack.push_front(nhead);
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
