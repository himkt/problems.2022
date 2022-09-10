fn swap(p: &mut [usize], i: usize, j: usize) -> String {
    let tmp = p[i];
    p[i] = p[j];
    p[j] = tmp;

    if j - i == 1 {
        format!("A {}", i + 1)
    } else {
        format!("B {}", i + 1)
    }

}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut p: Vec<usize> = scanner.vec(n);

    let mut hist = vec![];

    // i 番目の要素の値を i+1 にする
    for i in 0..n {
        if (i + 1) % 2 == p[i] % 2 {
            continue;
        }

        let mut target = None;
        for k in 0..n {
            // 偶奇が異なるペアを探す

            if i + 2 * k + 1 >= n {
                break;
            }

            if p[i] % 2 != p[i + 2 * k + 1] % 2 {
                target = Some(2 * k + 1);
                break;
            }
        }

        if let Some(mut c) = target {
            while c > 0 {
                if c > 1 {
                    let log = swap(&mut p, i + c - 2, i + c);
                    hist.push(log);
                    c -= 2;
                }
                else {
                    let log = swap(&mut p, i, i + c);
                    hist.push(log);
                    break;
                }
            }
        }
    }

    for i in 0..n {
        if (i + 1) != p[i] {
            for k in 1..n {
                if i + 2 * k >= n {
                    break;
                }
                if (i + 1) == p[i + 2 * k] {
                    let mut c = 2 * k;
                    while c > 0 {
                        let log = swap(&mut p, i + c - 2, i + c);
                        hist.push(log);
                        c -= 2;
                    }
                }
            }
        }
    }


    println!("{}", hist.len());
    for log in hist {
        println!("{}", log);
    }
}

use std::io::Write;
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
