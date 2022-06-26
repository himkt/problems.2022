#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = 9;

    let mut c: Vec<(usize, usize)> = (1..=m)
        .map(|i| {
            let ci: usize = scanner.cin();
            (ci, i)
        })
        .collect();
    c.sort_unstable_by_key(|&(ci, i)| (ci, Reverse(i)));

    let mut cc = vec![c[0]];
    for i in 1..m {
        if c[i].0 == cc.last().unwrap().0 {
            continue;
        }
        cc.push(c[i]);
    }

    let mut c = cc;
    let (c0, i0) = c[0];

    let mut vs = vec![];
    let mut xs = vec![];
    let mut sum = 0;
    while sum + c0 <= n {
        vs.push(i0);
        xs.push(c0);
        sum += c0;
    }

    c.sort_unstable_by_key(|&(_, i)| Reverse(i));

    let p = vs.len();
    let m = c.len();

    for k in 0..p {
        for q in 0..m {
            let (cj, j) = c[q];
            if xs[k] > cj {
                continue;
            }

            let diff = cj - c0;
            if sum + diff <= n {
                vs[k] = j;
                xs[k] = cj;
                sum += diff;
                break;
            }
        }
    }

    let ans: String = vs.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
    println!("{}", ans);
}

use std::{io::Write, cmp::Reverse};
pub fn flush() { std::io::stdout().flush().unwrap(); }
pub struct Scanner { stdin: std::io::Stdin, buffer: std::collections::VecDeque<String> }
impl Scanner {
    fn new() -> Self {
        Self { stdin: std::io::stdin(), buffer: std::collections::VecDeque::new() }
    }

    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            line.split_whitespace().for_each(|x| self.buffer.push_back(String::from(x)));
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}
