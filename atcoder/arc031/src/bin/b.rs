#[allow(clippy::needless_range_loop)]
fn search(src: (i32, i32), board: Vec<Vec<char>>) -> bool {
    let dxs: Vec<i32> = vec![0, 1, 0, -1];
    let dys: Vec<i32> = vec![1, 0, -1, 0];

    let mut board = board;
    let mut queue = VecDeque::new();
    queue.push_back(src);

    while let Some((cx, cy)) = queue.pop_front() {
        let cx_usize = cx as usize;
        let cy_usize = cy as usize;
        board[cx_usize][cy_usize] = 'x';

        for i in 0..4 {
            let nx = cx + dxs[i];
            let ny = cy + dys[i];

            if nx < 0 || ny < 0 {
                continue;
            }

            if 10 <= nx || 10 <= ny {
                continue;
            }

            let nx_usize = nx as usize;
            let ny_usize = ny as usize;
            if board[nx_usize][ny_usize] == 'o' {
                queue.push_back((nx, ny));
            }
        }
    }

    for i in 0..10 {
        for j in 0..10 {
            if board[i][j] == 'o' { return false; }
        }
    }

    true
}


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let board: Vec<Vec<char>> = (0..10)
        .map(|_| {
            let s: String = scanner.cin();
            let s: Vec<char> = s.chars().collect();
            s
        })
        .collect();

    let mut ans = false;
    for i in 0..10 {
        for j in 0..10 {
            if board[i][j] == 'o' { continue; }

            let src = (i as i32, j as i32);
            ans = ans || search(src, board.clone());
        }
    }

    if ans { println!("YES"); }
    else { println!("NO"); }
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
