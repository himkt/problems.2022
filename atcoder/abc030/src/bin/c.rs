#[derive(PartialEq, Eq)]
enum CurrentAirport { A, B }

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let x: usize = scanner.cin();
    let y: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);
    let b: Vec<usize> = scanner.vec(m);

    let mut t = 0;
    let mut state = CurrentAirport::A;
    let mut xid = 0;
    let mut yid = 0;

    let mut ans = 0;
    loop {
        if state == CurrentAirport::A && xid == n {
            break;
        }

        if state == CurrentAirport::B && yid == m {
            break;
        }

        match state {
            CurrentAirport::A => {
                loop {
                    if xid == n {
                        break;
                    }
                    if a[xid] < t {
                        xid += 1;
                        continue;
                    }
                    break;
                }

                if xid == n {
                    continue;
                }

                t = a[xid] + x;
                xid += 1;
                state = CurrentAirport::B;
            },
            CurrentAirport::B => {
                loop {
                    if yid == m {
                        break;
                    }
                    if b[yid] < t {
                        yid += 1;
                        continue;
                    }
                    break;
                }

                if yid == m {
                    continue;
                }

                t = b[yid] + y;
                state = CurrentAirport::A;
                yid += 1;
                ans += 1;
            },
        }
    }

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
