#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();
    let _: usize = scanner.cin();
    let mut s: Vec<Vec<char>> = (0..h)
        .map(|_| scanner.cin::<String>().chars().collect())
        .collect();

    for i in 0..h {
        for j in 0..(w - 1) {
            if s[i][j] == '#' && s[i][j + 1] == '.' {
                s[i].swap(j, j + 1);
            }
        }
    }

    let mut ans = vec![vec![0; w]; h];
    let mut idx = 1;
    let mut ci = 0;
    let mut cj = 0;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                for li in ci..=i {
                    for lj in cj..=j {
                        ans[li][lj] = idx;
                    }
                }
                idx += 1;
                if j == w - 1 {
                    ci = i + 1;
                    cj = 0;
                }
                else {
                    cj = j + 1;
                }
            }
        }
    }

    for j in 0..w {
        for i in 1..h {
            if ans[i][j] == 0 {
                ans[i][j] = ans[i - 1][j];
            }
        }
    }

    for i in 0..h {
        let row = ans[i].iter().map(|&x| x.to_string());
        let row: String = row.collect::<Vec<_>>().join(" ");
        println!("{}", row);
    }
}

use std::io::Write;
pub struct Scanner {
    buffer: std::collections::VecDeque<String>,
    buf: String,
}
#[allow(clippy::new_without_default)]
impl Scanner {
    pub fn new() -> Self {
        Scanner {
            buffer: std::collections::VecDeque::new(),
            buf: String::new(),
        }
    }
    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        if !self.buffer.is_empty() {
            return self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap();
        }
        self.buf.truncate(0);
        std::io::stdin().read_line(&mut self.buf).ok();
        self.buf
            .to_owned()
            .split_whitespace()
            .for_each(|x| self.buffer.push_back(String::from(x)));
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
    pub fn flush(&self) {
        std::io::stdout().flush().unwrap();
    }
}
