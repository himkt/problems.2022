#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();

    let board: Vec<Vec<char>> = (0..h)
        .map(|_| {
            let l: String = scanner.cin();
            let cs: Vec<char> = l.chars().collect();
            cs
        })
        .collect();

    let mut vertical:  Vec<Vec<usize>> = vec![vec![0; w]; h];
    let mut horizontal: Vec<Vec<usize>> = vec![vec![0; w]; h];

    for j in 0..w {
        if board[0][j] == '.' {
            vertical[0][j] = 1;
        }

        for i in 1..h {
            if board[i][j] == '.' {
                vertical[i][j] = vertical[i-1][j] + 1;
            }
        }
    }

    for j in 0..w {
        for i in (1..h).rev() {
            if vertical[i][j] > 0 && board[i-1][j] == '.' {
                vertical[i-1][j] = vertical[i][j];
            }
        }
    }

    for i in 0..h {
        if board[i][0] == '.' {
            horizontal[i][0] = 1;
        }

        for j in 1..w {
            if board[i][j] == '.' {
                horizontal[i][j] = horizontal[i][j-1] + 1;
            }
        }
    }

    for i in 0..h {
        for j in (1..w).rev() {
            if horizontal[i][j] > 0 && board[i][j-1] == '.' {
                horizontal[i][j-1] = horizontal[i][j];
            }
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if board[i][j] == '#' { continue; }
            ans = ans.max(vertical[i][j] + horizontal[i][j] - 1);
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
