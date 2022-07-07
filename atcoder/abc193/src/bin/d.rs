#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let k: usize = scanner.cin();
    let s: Vec<i64> = scanner.cin::<String>().chars().take(4).map(|x| x.to_string().parse::<i64>().unwrap()).collect();
    let t: Vec<i64> = scanner.cin::<String>().chars().take(4).map(|x| x.to_string().parse::<i64>().unwrap()).collect();

    let mut cnt = HashMap::new();
    let mut scnt: HashMap<i64, u32> = HashMap::new();
    let mut tcnt: HashMap<i64, u32> = HashMap::new();
    for i in 1..=9 {
        cnt.insert(i, k as i64);
        scnt.insert(i, 0);
        tcnt.insert(i, 0);
    }

    for k in 0..4 {
        cnt.entry(s[k]).and_modify(|x| *x -= 1);
        scnt.entry(s[k]).and_modify(|x| *x += 1);

        cnt.entry(t[k]).and_modify(|x| *x -= 1);
        tcnt.entry(t[k]).and_modify(|x| *x += 1);
    }

    let score = |mut kcnt: HashMap<i64, u32>, i: i64| -> i64 {
        kcnt.entry(i).and_modify(|x| *x += 1);
        (1..=9).map(|i| i * 10i64.pow(kcnt[&i])).sum()
    };

    let num_cards = (9 * k - 8) as f64;
    let mut prob = 0 as f64;

    for i in 1..=9 {
        for j in 1..=9 {
            if cnt[&i] == 0 {
                continue;
            }
            let p1 = cnt[&i] as f64 / num_cards;
            cnt.entry(i).and_modify(|x| *x -= 1);

            if cnt[&j] == 0 {
                cnt.entry(i).and_modify(|x| *x += 1);
                continue;
            }

            let sscore = score(scnt.clone(), i);
            let tscore = score(tcnt.clone(), j);

            if sscore > tscore {
                let p2 = cnt[&j] as f64 / (num_cards - 1.0);
                prob += p1 * p2;
            }

            cnt.entry(i).and_modify(|x| *x += 1);
        }
    }

    println!("{}", prob);
}

use std::{io::Write, collections::HashMap}; pub fn flush() { std::io::stdout().flush().unwrap(); }
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
