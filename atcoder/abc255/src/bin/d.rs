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


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let q: usize = scanner.cin();
    let mut a: Vec<i128> = scanner.vec(n);
    a.sort_unstable();

    let mut cumsum = a.clone();
    for i in 1..n {
        cumsum[i] += cumsum[i - 1];
    }

    for _ in 0..q {
        let x: i128 = scanner.cin();
        let mut ans = 0;

        // (a) x より小さい部分　
        if x >= a[0] {
            let below = lower_bound(
                0..n,
                &|q| a[q] >= x,
            );

            if below > 0 {
                ans += below as i128 * x - cumsum[below - 1];
            }
        }

        // (b) x より大きい部分
        let above = lower_bound(
            0..n,
            &|q| a[q] > x,
        );

        if above == 0 {
            ans += cumsum[n - 1] - (n as i128 * x);
        }
        else if above < n {
            let vsum = cumsum[n - 1] - cumsum[above - 1];
            ans += vsum - (n as i128 - above as i128) * x;
        }

        println!("{}", ans);
    }
}


use std::collections::VecDeque;
use std::io::{self, Write};
use std::str::FromStr;

pub struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
impl Scanner {
    fn new() -> Self {
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
    }

    pub fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

pub fn flush() {
    std::io::stdout().flush().unwrap();
}
