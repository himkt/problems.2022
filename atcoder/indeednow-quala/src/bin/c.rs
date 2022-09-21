const MAX: usize = 1_000_000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let mut num_people_at_the_score: Vec<usize> = vec![0; MAX + 1];
    let n: usize = scanner.cin();

    let mut max = 0;
    for _ in 0..n {
        let si: usize = scanner.cin();
        max = max.max(si);
        num_people_at_the_score[si] += 1;
    }
    for i in 0..MAX {
        num_people_at_the_score[i + 1] += num_people_at_the_score[i];
    }

    // cum[i] = i - 1 点以上の人の数
    let mut cum = vec![n; MAX + 2];
    for i in 0..=MAX {
        cum[i + 1] = n - num_people_at_the_score[i];
    }

    let q: usize = scanner.cin();
    for _ in 0..q {
        let ki: usize = scanner.cin();

        // キャパシティが 0 -> 誰も通過できないようにする
        // 全員が 0 点ならボーダーが存在しなくてもよい
        if ki == 0 {
            println!("{}", if max == 0 { 0 } else { max + 1 });
            continue;
        }

        let ri = lower_bound(
            0..(MAX + 1),
            &|x| cum[x] <= ki,
        );

        // ボーダーが 1 点な場合、ボーダーを撤廃しても通過者の数は変化しない
        // \since 0 点な候補者は条件より通過できない)
        println!("{}", if ri == 1 { 0 } else { ri });
    }
}

pub fn lower_bound(range: std::ops::Range<usize>, prop: &dyn Fn(usize) -> bool) -> usize {
    if prop(range.start) {
        return range.start;
    }

    let mut ng = range.start;
    let mut ok = range.end;

    while ok - ng > 1 {
        let middle = ng + (ok - ng) / 2;
        match prop(middle) {
            true => ok = middle,
            false => ng = middle,
        }
    }

    ok
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
