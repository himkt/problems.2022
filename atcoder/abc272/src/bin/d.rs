const INF: usize = 1_000_000_000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut dist = vec![vec![INF; n]; n];
    dist[0][0] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), 0));

    while let Some(((ci, cj), cost)) = queue.pop_front() {
        // println!("c=({}, {}), queue={:?}", ci, cj, queue);

        let mut p = 0;
        while p * p <= m {
            let di = p;
            let djf = m - di * di;
            let dj = (djf as f64).sqrt() as usize;

            if di * di + dj * dj != m {
                p += 1;
                continue;
            }

            // println!("di={}, dj={}", di, dj);

            // [ci + di, cj + dj]
            let ni = ci + di;
            let nj = cj + dj;

            if ni < n && nj < n && cost + 1 < dist[ni][nj] {
                // println!("ni={}, nj={}", ni, nj);
                dist[ni][nj] = cost + 1;
                queue.push_back(((ni, nj), cost + 1));
            }

            // [ci + di, cj - dj]
            if cj >= dj {
                let ni = ci + di;
                let nj = cj - dj;

                if ni < n && nj < n && cost + 1 < dist[ni][nj] {
                    // println!("ni={}, nj={}", ni, nj);
                    dist[ni][nj] = cost + 1;
                    queue.push_back(((ni, nj), cost + 1));
                }
            }

            // [ci - di, cj + dj]
            if ci >= di {
                let ni = ci - di;
                let nj = cj + dj;

                if ni < n && nj < n && cost + 1 < dist[ni][nj] {
                    // println!("ni={}, nj={}", ni, nj);
                    dist[ni][nj] = cost + 1;
                    queue.push_back(((ni, nj), cost + 1));
                }
            }

            // [ci - di, cj - dj]
            if ci >= di && cj >= dj {
                let ni = ci - di;
                let nj = cj - dj;

                if ni < n && nj < n && cost + 1 < dist[ni][nj] {
                    // println!("ni={}, nj={}", ni, nj);
                    dist[ni][nj] = cost + 1;
                    queue.push_back(((ni, nj), cost + 1));
                }
            }
            p += 1;
        }
    }

    for row in dist {
        let row: Vec<String> = row.iter().map(|&x| if x != INF { x.to_string() } else { "-1".to_string() }).collect();
        let row: String = row.join(" ");
        println!("{}", row);
    }
}

use std::{io::Write, collections::VecDeque};
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
