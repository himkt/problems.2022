#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut a: Vec<Vec<char>> = vec![vec!['-'; n]; n];

    for i in 0..n {
        let si: String = scanner.cin();
        let cs: Vec<char> = si.chars().collect();
        for j in 0..n {
            let aij = cs[j];
            a[i][j] = aij;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            let directions_x = vec!["+", "0", "-"];
            let directions_y = vec!["+", "0", "-"];
            for p in 0..3 {
                for q in 0..3 {
                    let direction_x = directions_x[p];
                    let direction_y = directions_y[q];
                    if direction_x == direction_y && direction_x == "0" {
                        continue;
                    }

                    let mut ci = i.clone();
                    let mut cj = j.clone();
                    let mut nums = vec![];
                    for _ in 0..n {
                        nums.push(a[ci][cj]);
                        if direction_x == "+" {
                            if ci == n - 1 {
                                ci = 0;
                            }
                            else {
                                ci += 1;
                            }
                        }
                        else if direction_x == "-" {
                            if ci == 0 {
                                ci = n - 1;
                            }
                            else {
                                ci -= 1;
                            }
                        }

                        if direction_y == "+" {
                            if cj == n - 1 {
                                cj = 0;
                            }
                            else {
                                cj += 1;
                            }
                        }
                        else if direction_y == "-" {
                            if cj == 0 {
                                cj = n - 1;
                            }
                            else {
                                cj -= 1;
                            }
                        }
                    }

                    let num: String = nums.iter().collect();
                    let num_i = num.parse::<usize>().unwrap();
                    ans = ans.max(num_i);
                }
            }
        }
    }

    println!("{}", ans);
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
