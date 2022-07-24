#[derive(Debug, Clone)]
pub struct SegmentTree {
    v: Vec<i64>,
    op: Op,
}

#[derive(Debug, Clone)]
pub enum Op {
    Max,
    Min,
    Add,
}

impl SegmentTree {
    const SEQ_LEN: usize = 1<<20;
    const MAX: i64 =   1_000_000_000_000;
    const MIN: i64 = - 1_000_000_000_000;

    pub fn new(op: Op) -> Self {
        let default = match &op {
            Op::Add => { 0 },
            Op::Max => { SegmentTree::MIN },
            Op::Min => { SegmentTree::MAX },
        };

        Self {
            v: vec![default; 2 * SegmentTree::SEQ_LEN],
            op,
        }
    }

    /// Update an i-th element to `value`.
    /// 0-origin.
    pub fn update_one(&mut self, mut index: usize, value: i64) {
        index += SegmentTree::SEQ_LEN;

        match &self.op {
            Op::Add => {
                self.v[index] += value;
            },
            _ => {
                self.v[index] = value;
            }
        };

        while index > 0 {
            index /= 2;
            let lv = self.v[index * 2];
            let rv = self.v[index * 2 + 1];

            match &self.op {
                Op::Add => {
                    self.v[index] = lv + rv;
                }
                Op::Max => {
                    self.v[index] = lv.max(rv);
                },
                Op::Min => {
                    self.v[index] = lv.min(rv);
                },
            };
        }
    }

    /// Run a range query on `[l, r)`. Note that it is a half-open interval.
    /// 0-origin.
    pub fn op(&self, l: usize, r: usize) -> i64 {
        self._op(l, r, 0, SegmentTree::SEQ_LEN, 1)
    }

    fn _op(&self, ql: usize, qr: usize, sl: usize, sr: usize, pos: usize) -> i64 {
        if qr <= sl || sr <= ql {
            return match &self.op {
                Op::Add => 0,
                Op::Max => SegmentTree::MIN,
                Op::Min => SegmentTree::MAX,
            };
        }
        if ql <= sl && sr <= qr {
            return self.v[pos];
        }

        let sm = (sl + sr) / 2;
        let lv = self._op(ql, qr, sl, sm, pos * 2);
        let rv = self._op(ql, qr, sm, sr, pos * 2 + 1);

        match &self.op {
            Op::Add => { lv + rv },
            Op::Max => { lv.max(rv) },
            Op::Min => { lv.min(rv) },
        }
    }
}


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<i64> = scanner.vec(n);

    let mut cum: Vec<i64> = a
        .iter()
        .map(|&x| match x { 0 => 1, 1 => -1, _ => panic!() })
        .collect();

    for i in 1..n {
        cum[i] += cum[i - 1];
    }

    let mut rminq = SegmentTree::new(Op::Min);
    let mut rmaxq = SegmentTree::new(Op::Max);

    for i in 0..n {
        rminq.update_one(i, cum[i]);
        rmaxq.update_one(i, cum[i]);
    }

    let mut max = cum.iter().cloned().max().unwrap();
    let mut min = cum.iter().cloned().min().unwrap();

    for i in (0..n).rev() {
        let lv = cum[i];
        let rv = rminq.op(0, i + 1);
        max = max.max(lv - rv);

        let lv = cum[i];
        let rv = rmaxq.op(0, i + 1);
        min = min.min(lv - rv);
    }

    let ans = max - min + 1;
    println!("{}", ans);
}

use std::io::Write; pub fn flush() { std::io::stdout().flush().unwrap(); }
pub struct Scanner { buffer: std::collections::VecDeque<String>, buf: String }
impl Scanner {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self { Scanner { buffer: std::collections::VecDeque::new(), buf: String::new() } }
    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        if !self.buffer.is_empty() { return self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap(); }
        self.buf.truncate(0); std::io::stdin().read_line(&mut self.buf).ok();
        self.buf.to_owned().split_whitespace().for_each(|x| self.buffer.push_back(String::from(x)));
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> { (0..n).map(|_| self.cin()).collect() }
}
