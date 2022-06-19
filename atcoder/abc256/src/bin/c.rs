#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let h: Vec<usize> = scanner.vec(3);
    let w: Vec<usize> = scanner.vec(3);

    let mut ans = 0;
    for a11 in 1..h[0] {
        for a12 in 1..h[0] {
            if a11 + a12 >= h[0] {
                continue;
            }
            let a13 = h[0] - a11 - a12;

            for a21 in 1..h[1] {
                for a22 in 1..h[1] {
                    if a21 + a22 >= h[1] {
                        continue;
                    }

                    let a23 = h[1] - a21 - a22;

                    if a11 + a21 >= w[0] {
                        continue;
                    }

                    if a12 + a22 >= w[1] {
                        continue;
                    }

                    if a13 + a23 >= w[2] {
                        continue;
                    }

                    let a31 = w[0] - a11 - a21;
                    let a32 = w[1] - a12 - a22;
                    let a33 = w[2] - a13 - a23;

                    if a11 + a12 + a13 != h[0] {
                        continue;
                    }

                    if a21 + a22 + a23 != h[1] {
                        continue;
                    }

                    if a31 + a32 + a33 != h[2] {
                        continue;
                    }

                    if a11 + a21 + a31 != w[0] {
                        continue;
                    }

                    if a12 + a22 + a32 != w[1] {
                        continue;
                    }

                    if a13 + a23 + a33 != w[2] {
                        continue;
                    }

                    ans += 1;
                }
            }
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
