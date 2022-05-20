const INF: usize = 100_000_000_000_000;


#[allow(clippy::needless_range_loop)]
#[allow(clippy::collapsible_else_if)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let clues: Vec<(usize, usize, usize)> = (0..n)
        .map(|_| {
            let x: usize = scanner.cin();
            let y: usize = scanner.cin();
            let h: usize = scanner.cin();
            (x, y, h)
        })
        .collect();

    if n == 1 {
        let (cx, cy, h) = clues[0];
        println!("{} {} {}", cx, cy, h);
        return;
    }

    if n == 2 {
        let (cx1, cy1, h1) = clues[0];
        let (cx2, cy2, h2) = clues[1];
        if h1 > h2 {
            println!("{} {} {}", cx1, cy1, h1);
        }
        else {
            println!("{} {} {}", cx2, cy2, h2);
        }
        return;
    }

    let template_board = vec![vec![INF; 101]; 101];

    for cx in 0..=100 {
        for cy in 0..=100 {
            let mut board = template_board.clone();
            let mut yh = INF;

            for i in 0..n {
                let (xx, xy, xh) = clues[i];
                board[xx][xy] = xh;

                let dx = cx.max(xx) - cx.min(xx);
                let dy = cy.max(xy) - cy.min(xy);
                yh = yh.min(xh + dx + dy);
            }

            let mut solved = true;
            for zx in 0..=100 {
                for zy in 0..=100 {
                    let dx = cx.max(zx) - cx.min(zx);
                    let dy = cy.max(zy) - cy.min(zy);

                    let zh = if yh < dx + dy {
                        0
                    }
                    else {
                        yh - dx - dy
                    };

                    if board[zx][zy] == INF {
                        board[zx][zy] = zh;
                    }
                    else {
                        if board[zx][zy] != zh {
                            solved = false;
                            break;
                        }
                    }
                }

                if !solved {
                    break;
                }
            }

            if solved {
                println!("{} {} {}", cx, cy, yh);
                return;
            }
        }
    }
}


use std::collections::VecDeque;
use std::io::{self, Write};
use std::str::FromStr;

pub struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
impl Scanner {
    fn new() -> Self {
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
    }

    pub fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

pub fn flush() {
    std::io::stdout().flush().unwrap();
}
