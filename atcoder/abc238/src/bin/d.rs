#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let t: usize = scanner.cin();

    for _ in 0..t {
        let mut a: usize = scanner.cin();
        let mut s: usize = scanner.cin();
        let ao = a.clone();
        let so = s.clone();
        
        let mut carried = false;

        let mut x: usize = 0;
        let mut y: usize = 0;

        for i in 0..62 {
            let ai = a % 2;
            let si = s % 2;

            if a > 0 && s == 0 {
                break;
            }

            if s == 0 {
                break;
            }

            if !carried {
                if ai == 1 {
                    if si == 1 {
                        break;
                    }
                    else {
                        // (xi, yi) = (1, 1)
                        carried = true;

                        x += 2usize.pow(i);
                        y += 2usize.pow(i);
                    }
                }
                else {
                    if si == 1 {
                        // (xi, yi) = (1, 0) or (0, 1)
                        carried = false;

                        x += 2usize.pow(i);
                    }
                    else {
                        // (xi, yi) = (0, 0)
                        carried = false;
                    }
                }
            }
            // if carried
            else {
                if ai == 1 {
                    if si == 1 {
                        // (xi, yi) = (1, 1)
                        carried = true;

                        x += 2usize.pow(i);
                        y += 2usize.pow(i);
                    }
                    else {
                        break;
                    }
                }
                else {
                    if si == 1 {
                        // (xi, yi) = (0, 0)
                        carried = false;
                    }
                    else {
                        // (xi, yi) = (1, 0) or (0, 1)
                        carried = true;

                        x += 2usize.pow(i);
                    }
                }
            }

            a>>=1;
            s>>=1;
        }

        if x + y == so && x & y == ao {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
}

use std::io::Write; pub fn flush() { std::io::stdout().flush().unwrap(); }
pub struct Scanner { buffer: std::collections::VecDeque<String>, buf: String }
impl Scanner {
    pub fn new() -> Self { Scanner { buffer: std::collections::VecDeque::new(), buf: String::new() } }
    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        if !self.buffer.is_empty() { return self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap(); }
        self.buf.truncate(0); std::io::stdin().read_line(&mut self.buf).ok();
        self.buf.to_owned().split_whitespace().for_each(|x| self.buffer.push_back(String::from(x)));
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> { (0..n).map(|_| self.cin()).collect() }
}
