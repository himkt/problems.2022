#[derive(PartialEq,Eq)]
enum Direction { R, L }

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);
    let mut c: Vec<usize> = vec![0; n];

    let mut board = vec![vec![0; w]; h];
    let mut state = Direction::R;

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    for _ in 0..(h * w) {
       board[i][j] = k + 1;
       c[k] += 1;

       if a[k] == c[k] {
           k += 1;
       }

       if j == (w - 1) && state == Direction::R {
           i += 1;
           state = Direction::L;
           continue;
       }
       if j == 0 && state == Direction::L {
           i += 1;
           state = Direction::R;
           continue;
       }

       match state {
           Direction::R => j += 1,
           Direction::L => j -= 1,
       }
    }

    for row in board {
        let v: String = row.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", v);
    }
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
