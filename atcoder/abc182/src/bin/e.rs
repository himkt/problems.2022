#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();

    let h: usize = scanner.cin();
    let w: usize = scanner.cin();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut board = vec![vec![-1; w]; h];

    for _ in 0..n {
        let a: usize = scanner.cin();
        let b: usize = scanner.cin();
        board[a-1][b-1] = 1;
    }

    for _ in 0..m {
        let c: usize = scanner.cin();
        let d: usize = scanner.cin();
        board[c-1][d-1] = 0;
    }

    let mut board_row = board.clone();
    let mut board_col = board.clone();

    for i in 0..h {
        for j in 1..w {
            if board_row[i][j-1] == 1 && board_row[i][j] == -1 {
                board_row[i][j] = 1;
            }
        }
    }

    for i in 0..h {
        for j in (0..w-1).rev() {
            if board_row[i][j+1] == 1 && board_row[i][j] == -1 {
                board_row[i][j] = 1;
            }
        }
    }

    for j in 0..w {
        for i in 1..h {
            if board_col[i-1][j] == 1 && board_col[i][j] == -1 {
                board_col[i][j] = 1;
            }
        }
    }

    for j in 0..w {
        for i in (0..h-1).rev() {
            if board_col[i+1][j] == 1 && board_col[i][j] == -1 {
                board_col[i][j] = 1;
            }
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if board_row[i][j] == 1 || board_col[i][j] == 1 {
                ans += 1;
            }
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

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

#[allow(dead_code)]
fn flush() {
    std::io::stdout().flush().unwrap();
}
