fn find_zero(board: &[Vec<usize>], h: usize, w: usize) -> Option<(usize, usize)> {
    for i in 0..h {
        for j in 0..w {
            if board[i][j] == 0 {
                return Some((i, j));
            }
        }
    }
    None
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();
    let a: usize = scanner.cin();
    let b: usize = scanner.cin();

    let board = vec![vec![0; w]; h];
    let mut stack = VecDeque::new();
    stack.push_back((board.clone(), a, b));

    let mut visited = HashSet::new();
    let mut ans = 0;

    while let Some((board, na, nb)) = stack.pop_front() {
        if let Some((i, j)) = find_zero(&board, h, w) {
            if board[i][j] == 0 {
                let mut board_i = board.clone();
                if na > 0 && j + 1 < w && board_i[i][j + 1] == 0 {
                    board_i[i][j] = 1;
                    board_i[i][j + 1] = 1;

                    if !visited.contains(&board_i) {
                        stack.push_back((board_i.clone(), na - 1, nb));
                        visited.insert(board_i);
                    }
                }

                let mut board_j = board.clone();
                if na > 0 && i + 1 < h && board_j[i + 1][j] == 0 {
                    board_j[i][j] = 2;
                    board_j[i + 1][j] = 2;

                    if !visited.contains(&board_j) {
                        stack.push_back((board_j.clone(), na - 1, nb));
                        visited.insert(board_j);
                    }
                }

                if nb > 0 {
                    let mut board_k = board.clone();
                    board_k[i][j] = 3;

                    if !visited.contains(&board_k) {
                        stack.push_back((board_k.clone(), na, nb - 1));
                        visited.insert(board_k);
                    }
                }
            }
        }
        else {
            ans += 1;
        }
    }

    println!("{}", ans);
}

use std::{io::Write, collections::{VecDeque, HashSet}};
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
