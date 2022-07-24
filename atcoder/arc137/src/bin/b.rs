#[derive(Debug, Clone)]
pub struct SegmentTree {
    v: Vec<i64>,
    mode: Mode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    RangeUpdate(Op),
    RangeGet(Op),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Max,
    Min,
    Add,
}

// Segment tree implementation. All operations are 0-origin.
// Note that a half-open interval [l, r) is used as a range representation.
impl SegmentTree {
    const SEQ_LEN: usize = 1 << 20;
    const MAX: i64 = 1_000_000_000_000;
    const MIN: i64 = -1_000_000_000_000;

    pub fn new(mode: Mode) -> Self {
        let default = match &mode {
            Mode::RangeGet(op) => SegmentTree::default(op),
            Mode::RangeUpdate(op) => SegmentTree::default(op),
        };

        Self {
            v: vec![default; 2 * SegmentTree::SEQ_LEN],
            mode,
        }
    }

    /// Return an appropriate default value for the given operation.
    pub fn default(op: &Op) -> i64 {
        match op {
            Op::Add => 0,
            Op::Max => SegmentTree::MIN,
            Op::Min => SegmentTree::MAX,
        }
    }

    pub fn _assign(ret: &mut i64, value: i64) {
        *ret = value;
    }

    pub fn _add(lv: i64, rv: i64) -> i64 {
        lv + rv
    }

    pub fn _add_assign_one(ret: &mut i64, value: i64) {
        *ret += value;
    }

    pub fn _add_assign(ret: &mut i64, lv: i64, rv: i64) {
        *ret = lv + rv;
    }

    pub fn _max(lv: i64, rv: i64) -> i64 {
        lv.max(rv)
    }

    pub fn _max_assign(ret: &mut i64, lv: i64, rv: i64) {
        *ret = lv.max(rv);
    }

    pub fn _min(lv: i64, rv: i64) -> i64 {
        lv.min(rv)
    }

    pub fn _min_assign(ret: &mut i64, lv: i64, rv: i64) {
        *ret = lv.min(rv);
    }

    /// Get an i-th element of from the tree.
    pub fn get_one(&mut self, mut index: usize) -> i64 {
        index += SegmentTree::SEQ_LEN;
        let mut ret = 0;

        if let Mode::RangeUpdate(op) = &self.mode {
            let operator = match op {
                Op::Add => SegmentTree::_add_assign_one,
                _ => panic!(),
            };

            operator(&mut ret, self.v[index]);
            while index > 0 {
                index /= 2;
                operator(&mut ret, self.v[index]);
            }
        }
        else {
            panic!("Unsupported");
        }

        ret
    }

    /// Run a range query.
    pub fn get_range(&self, l: usize, r: usize) -> i64 {
        fn _get_range(
            op: &Op,
            v: &Vec<i64>,
            ql: usize,
            qr: usize,
            sl: usize,
            sr: usize,
            pos: usize,
        ) -> i64 {
            if qr <= sl || sr <= ql {
                return SegmentTree::default(op);
            }

            if ql <= sl && sr <= qr {
                return v[pos];
            }

            let sm = (sl + sr) / 2;
            let lv = _get_range(op, v, ql, qr, sl, sm, pos * 2);
            let rv = _get_range(op, v, ql, qr, sm, sr, pos * 2 + 1);
            let operate = match op {
                Op::Add => SegmentTree::_add,
                Op::Max => SegmentTree::_max,
                Op::Min => SegmentTree::_min,
            };
            operate(lv, rv)
        }

        if let Mode::RangeGet(op) = &self.mode {
            let data = &self.v;
            _get_range(op, data, l, r, 0, SegmentTree::SEQ_LEN, 1)
        } else {
            panic!("Unsupported");
        }
    }

    /// Update an i-th element to `value`.
    pub fn update_one(&mut self, mut index: usize, value: i64) {
        index += SegmentTree::SEQ_LEN;

        if let Mode::RangeGet(op) = &self.mode {
            let operate_and_assign_one = match op {
                Op::Add => SegmentTree::_add_assign_one,
                Op::Max => SegmentTree::_assign,
                Op::Min => SegmentTree::_assign,
            };
            operate_and_assign_one(&mut self.v[index], value);

            let operate_and_assign = match op {
                Op::Add => SegmentTree::_add_assign,
                Op::Max => SegmentTree::_max_assign,
                Op::Min => SegmentTree::_min_assign,
            };

            while index > 0 {
                index /= 2;
                let lv = self.v[index * 2];
                let rv = self.v[index * 2 + 1];
                operate_and_assign(&mut self.v[index], lv, rv);
            }
        }
    }

    /// Add `value` to the range `[l, r)`.
    pub fn update_range(&mut self, mut l: usize, mut r: usize, value: i64) {
        if let Mode::RangeUpdate(op) = &self.mode {
            l += SegmentTree::SEQ_LEN;
            r += SegmentTree::SEQ_LEN;

            let operate_and_assign_one = match op {
                Op::Add => SegmentTree::_add_assign_one,
                _ => panic!(),
            };

            while l < r {
                if l % 2 == 1 {
                    operate_and_assign_one(&mut self.v[l], value);
                    l += 1;
                }
                l /= 2;

                if r % 2 == 1 {
                    operate_and_assign_one(&mut self.v[r - 1], value);
                    r -= 1;
                }
                r /= 2;
            }
        } else {
            panic!("Unsupported");
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

    let mut rminq = SegmentTree::new(Mode::RangeGet(Op::Min));
    let mut rmaxq = SegmentTree::new(Mode::RangeGet(Op::Max));

    for i in 0..n {
        rminq.update_one(i, cum[i]);
        rmaxq.update_one(i, cum[i]);
    }

    let mut max = cum.iter().cloned().max().unwrap();
    let mut min = cum.iter().cloned().min().unwrap();

    for i in (0..n).rev() {
        let lv = cum[i];
        let rv = rminq.get_range(0, i + 1);
        max = max.max(lv - rv);

        let lv = cum[i];
        let rv = rmaxq.get_range(0, i + 1);
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
