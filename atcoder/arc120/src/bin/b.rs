const DIV: usize = 998244353;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();

    let board: Vec<Vec<char>> = (0..h)
        .map(|_| scanner.cin::<String>().chars().collect())
        .collect();

    let mut ans = 1;
    let n: usize = h.max(w);
    for p in 0..(2 * n) {
        let mut counter = HashMap::new();
        counter.insert('R', 0);
        counter.insert('B', 0);
        counter.insert('.', 0);

        let mut i = p;
        let mut j = 0;
        while j < h + w {
            if i < h && j < w {
                counter.entry(board[i][j]).and_modify(|v| *v += 1);
            }

            if i == 0 {
                break;
            }

            i -= 1;
            j += 1;
        }

        let cr = counter[&'R'];
        let cb = counter[&'B'];
        let cc = counter[&'.'];

        if cr == cb && cb == cc && cc == 0 {
            continue;
        }

        if p == 0 || p == n {
            if cr == 0 && cb == 0 {
                ans *= 2;
            }
            else {
                ans *= 1;
            }
        }
        else {
            if cr > 0 && cb > 0 {
                ans *= 0;
            }
            else if cr == 0 && cb == 0 {
                ans *= 2;
            }
            else {
                ans *= 1;
            }
        }

        ans %= DIV;
    }

    ans %= DIV;
    println!("{}", ans);
}

use std::{io::Write, collections::HashMap};
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
