#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let xs = vec!['0', '2', '3', '4', '5', '6', '7', '8', '9'];

    let s: String = n.to_string();
    let d: usize = s.len();

    let mut ans = 0;
    for fx in 1..=d {
        //println!("# [fx={}]", fx);

        // fx = k
        // 11...1
        let l = (0..fx)
            .map(|_| '1')
            .collect::<String>()
            .parse()
            .unwrap();

        if n >= l {
            //println!("[1] {}, add {}", l, fx);
            ans += fx;
        }

        // fx = k - 1
        // 11...1x
        if fx > 1 {
            let l: usize = (0..fx)
                .map(|p| if p + 1 < fx { '1' } else { '0' })
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            let r: usize = (0..fx)
                .map(|p| if p + 1 < fx { '1' } else { '9' })
                .collect::<String>()
                .parse::<usize>().unwrap()
                .min(n);

            if r >= l {
                let freq = if r % 10 <= 1 { 1 } else { r % 10 };
                ans += (fx - 1) * freq;
                //println!("[2] l={}, r={}, freq={}, add {}", l, r, freq, (fx - 1)  * freq);
            }
        }

        // fx <= k - i (i >= 2) 
        // 11..xyy
        if fx > 2 {
            for score in 1..=fx {
                if score > fx {
                    continue;
                }
                if fx - score == 1 {
                    continue;
                }
                if fx - score == 0 {
                    continue;
                }
                //println!("score={}", score);
                for &x in xs.iter() {
                    let l: usize = (0..fx)
                        .map(|p| if p < score { '1' } else if p == score { x } else { '0' })
                        .collect::<String>().parse::<usize>().unwrap();

                    let r: usize = (0..fx)
                        .map(|p| if p < score { '1' } else if p == score { x } else { '9' })
                        .collect::<String>().parse::<usize>().unwrap()
                        .min(n);

                    if l > r {
                        continue;
                    }
                    //println!("[3] l={}, r={}, add {}", l, r, score * (r - l + 1));
                    ans += score * (r - l + 1);
                }
            }
        }
    }

    println!("{}", ans);
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
