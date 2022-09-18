pub fn lower_bound(range: std::ops::Range<usize>, prop: &dyn Fn(usize) -> bool) -> usize {
    if prop(range.start) {
        return range.start;
    }

    let mut ng = range.start;
    let mut ok = range.end;

    while ok - ng > 1 {
        let middle = ng + (ok - ng) / 2;
        match prop(middle) {
            true => ok = middle,
            false => ng = middle,
        }
    }

    ok
}

fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let mut ans = 0;
    let mut q = 1;
    let mut upper = n;

    loop {
        let li = lower_bound(
            0..n,
            &|x| {
                let y = n - x;
                n / y > q
            },
        );
        let lower = n - li + 1;
        ans += (upper - lower + 1) * q;

        if lower == 1 {
            break;
        }

        upper = lower - 1;
        q = n / (lower - 1);
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
