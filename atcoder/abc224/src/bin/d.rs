use std::collections::HashMap;

const NUM_CELL: usize = 9;
const EMPTY_CELL: usize = 8;


fn main() {
    let mut scanner = Scanner::new();
    let m: usize = scanner.cin();

    let mut g: Vec<Vec<usize>> = vec![vec![]; NUM_CELL];

    for _ in 0..m {
        let u: usize = scanner.cin();
        let v: usize = scanner.cin();

        g[u - 1].push(v - 1);
        g[v - 1].push(u - 1);
    }

    let mut board = vec![8usize; NUM_CELL];
    for i in 0..8 {
        let pi: usize = scanner.cin();
        board[pi - 1] = i;
    }

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(board.clone());

    let mut visited: HashMap::<Vec<usize>, usize> = HashMap::new();
    visited.insert(board, 0);

    while let Some(front) = queue.pop_front() {

        let empty_cell_id = front
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == EMPTY_CELL)
            .next()
            .unwrap()
            .0;

        for &j in &g[empty_cell_id] {
            let mut next_board = front.clone();
            next_board.swap(empty_cell_id, j);

            if visited.contains_key(&next_board) {
                continue;
            }

            queue.push_back(next_board.clone());
            let v = *visited.get(&front).unwrap();
            visited.insert(next_board, v + 1);
        }
    }

    let destination: Vec<usize> = (0..9).collect();
    if let Some(v) = visited.get(&destination) {
        println!("{}", v);
    }
    else {
        println!("-1");
    }
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

    fn usize1(&mut self) -> usize {
        self.cin::<usize>() - 1
    }

    fn chars(&mut self) -> Vec<char> {
        self.cin::<String>().chars().collect()
    }

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

#[allow(dead_code)]
fn flush() {
    std::io::stdout().flush().unwrap();
}
