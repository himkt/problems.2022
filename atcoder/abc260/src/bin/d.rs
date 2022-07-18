#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();
    let p: Vec<usize> = scanner.vec::<usize>(n).iter().map(|&x| x - 1).collect();

    let mut head_cards = BTreeSet::new();
    let mut cards = vec![vec![]; n];
    let mut eat = vec![-1; n];

    if k == 1 {
        eat[p[0]] = 1;
    }
    else {
        head_cards.insert(p[0]);
        cards[p[0]].push(p[0]);
    }

    for i in 1..n {
        if let Some(&current_head) = head_cards.range(p[i]..).next() {
            head_cards.remove(&current_head);
            head_cards.insert(p[i]);

            cards[current_head].push(p[i]);
            let p1: *mut Vec<usize> = &mut cards[current_head];
            let p2: *mut Vec<usize> = &mut cards[p[i]];
            unsafe {
                p1.swap(p2);
            }
        }
        else {
            head_cards.insert(p[i]);
            cards[p[i]].push(p[i]);
        }

        if cards[p[i]].len() == k {
            for &card in cards[p[i]].iter() {
                eat[card] = (i + 1) as i64;
            }
            head_cards.remove(&p[i]);
            cards[p[i]] = vec![];
        }
    }

    for e in eat {
        println!("{}", e);
    }
}

use std::{io::Write, collections::BTreeSet}; pub fn flush() { std::io::stdout().flush().unwrap(); }
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

