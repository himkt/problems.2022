#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut cnt_odd = HashMap::new();
    let mut cnt_evn = HashMap::new();

    for i in 0..n {
        if i % 2 == 0 {
            *cnt_odd.entry(a[i]).or_insert(0) += 1;
        }
        else {
            *cnt_evn.entry(a[i]).or_insert(0) += 1;
        }
    }

    let mut vs_odd: Vec<(usize, usize)> = cnt_odd.into_iter().collect();
    vs_odd.sort_unstable_by_key(|&(_, v)| Reverse(v));
    let mut vs_evn: Vec<(usize, usize)> = cnt_evn.into_iter().collect();
    vs_evn.sort_unstable_by_key(|&(_, v)| Reverse(v));

    if vs_odd.len() == 1 && vs_evn.len() == 1 {
        if vs_odd[0].0 == vs_evn[0].0 {
            println!("{}", vs_odd[0].1);
        }
        else {
            println!("0");
        }
    }
    else {
        let half = n / 2;
        if vs_odd.len() == 1 {
            if vs_odd[0].0 == vs_evn[0].0 {
                println!("{}", half - vs_evn[1].1);
            }
            else {
                println!("{}", half - vs_evn[0].1);
            }
        }
        else if vs_evn.len() == 1 {
            if vs_evn[0].0 == vs_odd[0].0 {
                println!("{}", half - vs_odd[1].1);
            }
            else {
                println!("{}", half - vs_odd[0].1);
            }
        }
        else {
            let mut ans = n;
            for i in 0..2 {
                for j in 0..2 {
                    let v_odd = vs_odd[i].0;
                    let v_evn = vs_evn[j].0;

                    if v_odd == v_evn {
                        continue;
                    }

                    ans = ans.min(n - vs_odd[i].1 - vs_evn[j].1);
                }
            }
            println!("{}", ans);
        }
    }
}

use std::{io::Write, collections::HashMap, cmp::Reverse};
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
