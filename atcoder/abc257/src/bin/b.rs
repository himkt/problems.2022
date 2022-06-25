#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();
    let q: usize = scanner.cin();

    let mut board: Vec<usize> = vec![0; n];
    for ai in 1..=k {
        let i: usize = scanner.cin();
        board[i - 1] = ai;
    }

    let find_lth = |board: &Vec<usize>, l: usize| -> usize {
        let mut ith = 1;

        for i in 0..n {
            if board[i] != 0 {
                if ith == l {
                    return i;
                }
                else {
                    ith += 1;
                }
            }
        }

        panic!();
    };

    for _ in 0..q {
        let l: usize = scanner.cin();
        let i = find_lth(&board, l);

        if i == n - 1 {
            continue;
        }

        if board[i + 1] != 0 {
            continue;
        }

        board[i + 1] = board[i];
        board[i] = 0;
    }

    let mut pos = vec![0; k + 1];
    for (i, &ai) in board.iter().enumerate() {
        if ai == 0 {
            continue;
        }
        pos[ai] = i + 1;
    }

    let ans = pos[1..]
        .iter()
        .map(|&v| v.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", ans);
}

use std::io::Write;
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
