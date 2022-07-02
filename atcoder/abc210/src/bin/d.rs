const INF: i64 = 100_000_000_000_000_000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();
    let c: i64 = scanner.cin();

    let a: Vec<Vec<i64>> = (0..h)
        .map(|_| scanner.vec::<i64>(w))
        .collect();

    let mut dp1 = vec![vec![INF; w]; h];

    let hi = h as i64;
    let wi = w as i64;

    dp1[h - 1][w - 1] = a[h - 1][w - 1] + c * ((hi - 1) + (wi - 1));

    for i in (0..(h - 1)).rev() {
        let ii = i as i64;
        dp1[i][w - 1] = dp1[i + 1][w - 1].min(a[i][w - 1] + c * (ii + wi - 1));
    }

    for j in (0..(w - 1)).rev() {
        let ji = j as i64;
        dp1[h - 1][j] = dp1[h - 1][j + 1].min(a[h - 1][j] + c * (hi - 1 + ji));
    }

    for i in (0..(h - 1)).rev() {
        let ii = i as i64;
        for j in (0..(w - 1)).rev() {
            let ji = j as i64;
            dp1[i][j] = a[i][j] + c * (ii + ji);
            dp1[i][j] = dp1[i][j].min(dp1[i + 1][j]);
            dp1[i][j] = dp1[i][j].min(dp1[i][j + 1]);
        }
    }

    let mut ans: i64 = INF;
    for i in 0..h {
        let ii = i as i64;

        for j in 0..w {
            if i == h - 1 && j == w - 1 {
                continue;
            }

            let ji = j as i64;
            let v1 = a[i][j] - c * (ii + ji);

            let v2 = if i == h - 1 {
                dp1[i][j + 1]
            }
            else if j == w - 1 {
                dp1[i + 1][j]
            }
            else {
                dp1[i + 1][j].min(dp1[i][j + 1])
            };

            ans = ans.min(v1 + v2);
        }
    }


    let mut dp2 = vec![vec![INF; w]; h];
    dp2[0][w - 1] = a[0][w - 1] - c * (0 - (wi - 1));

    for i in 1..h {
        let ii = i as i64;
        dp2[i][w - 1] = dp2[i - 1][w - 1].min(a[i][w - 1] - c * (ii - (wi - 1)));
    }

    for j in (0..(w - 1)).rev() {
        let ji = j as i64;
        dp2[0][j] = dp2[0][j + 1].min(a[0][j] - c * (0 - ji));
    }

    for i in 1..h {
        let ii = i as i64;
        for j in (0..(w - 1)).rev() {
            let ji = j as i64;
            dp2[i][j] = a[i][j] - c * (ii - ji);
            dp2[i][j] = dp2[i][j].min(dp2[i - 1][j]);
            dp2[i][j] = dp2[i][j].min(dp2[i][j + 1]);
        }
    }

    for i in 0..h {
        let ii = i as i64;

        for j in 0..w {
            if i == 0 && j == w - 1 {
                continue;
            }

            let ji = j as i64;
            let v1 = a[i][j] + c * (ii - ji);

            let v2 = if i == 0 {
                dp2[i][j + 1]
            }
            else if j == w - 1 {
                dp2[i - 1][j]
            }
            else {
                dp2[i - 1][j].min(dp2[i][j + 1])
            };

            ans = ans.min(v1 + v2);
        }
    }

    println!("{}", ans);
}

use std::io::Write; pub fn flush() { std::io::stdout().flush().unwrap(); }
pub struct Scanner { buffer: std::collections::VecDeque<String>, buf: String }
impl Scanner {
    pub fn new() -> Self { Scanner { buffer: std::collections::VecDeque::new(), buf: String::new() } }
    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        if !self.buffer.is_empty() { return self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap(); }
        self.buf.truncate(0); std::io::stdin().read_line(&mut self.buf).ok();
        self.buf.to_owned().split_whitespace().for_each(|x| self.buffer.push_back(String::from(x)));
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> { (0..n).map(|_| self.cin()).collect() }
}
