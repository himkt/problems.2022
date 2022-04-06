fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let l: usize = scanner.cin();
    let w: usize = scanner.cin();
    let mut a: Vec<usize> = scanner.vec(n);
    a.sort_unstable();

    let mut b: Vec<(usize, usize)> = vec![];
    b.push((0, a[0]));

    let mut c = 0;
    for ai in a {
        if c < ai {
            b.push((c, ai));
        }
        c = ai + w;
    }

    if c < l {
        b.push((c, l));
    }

    let mut ans = 0;
    let mut max_covered = 0;

    // println!("{:?}", b);

    for (mut s, t) in b {
        if s < max_covered {
            s = max_covered;
        }

        if t <= s {
            continue;
        }

        let k = ((t - s) + w - 1) / w;
        ans += k;
        // println!("s={}, t={}, k={}", s, t, k);

        max_covered = s + w * k;
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
