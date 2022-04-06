pub fn lower_bound(range: std::ops::Range<usize>, prop: &dyn Fn(usize) -> bool) -> usize {
    if prop(range.start) {
        range.start
    }
    else {
        let mut ng = range.start;
        let mut ok = range.end;

        while ok - ng > 1 {
            let middle = ng + (ok - ng) / 2;

            if prop(middle) {
                ok = middle;
            }
            else {
                ng = middle;
            }
        }

        ok
    }
}


fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let p: Vec<usize> = scanner.vec(n);
    let mut q: Vec<usize> = vec![0; n+1];

    for i in 0..n {
        let qi: usize = scanner.cin();
        q[qi] = i;
    }

    let mut vs = vec![];
    for i in 0..n {
        let mut j = p[i];
        while j <= n {
            vs.push((i, q[j]));
            j += p[i];
        }
    }

    vs.sort_unstable_by_key(|&(i1, i2)| (i1, Reverse(i2)));

    let mut lis = vec![];
    for (_, s) in vs {
        if lis.is_empty() { lis.push(s); }
        else {
            if &s > lis.last().unwrap() {
                lis.push(s);
            }
            else {
                let i = lower_bound(0..lis.len(), &|x| lis[x] >= s);
                lis[i] = s;
            }
        }
    }

    println!("{}", lis.len());
}


use std::cmp::Reverse;
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
