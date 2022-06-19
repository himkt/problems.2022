#[derive(Debug, Clone)]
pub struct RMaxQ {
    v: Vec<i64>,
}

impl Default for RMaxQ {
    fn default() -> Self {
        RMaxQ::new()
    }
}

impl RMaxQ {
    const SEQ_LEN: usize = 1<<19;
    const UNIT_VAL: i64 = - 1_000_000_000;

    pub fn new() -> Self {
        Self {
            v: vec![RMaxQ::UNIT_VAL; 2 * RMaxQ::SEQ_LEN],
        }
    }

    /// Add `value` to i-th element.
    /// 0-origin.
    pub fn update(&mut self, mut index: usize, value: i64) {
        index += RMaxQ::SEQ_LEN;
        self.v[index] = value;

        while index > 0 {
            index /= 2;
            self.v[index] = self.v[index * 2].max(self.v[index * 2 + 1]);
        }
    }

    /// Sum values on `[l, r)`. Note that it is a half-open interval.
    /// 0-origin.
    pub fn max(&self, l: usize, r: usize) -> i64 {
        self._max(l, r, 0, RMaxQ::SEQ_LEN, 1)
    }

    fn _max(&self, ql: usize, qr: usize, sl: usize, sr: usize, pos: usize) -> i64 {
        if qr <= sl || sr <= ql {
            return RMaxQ::UNIT_VAL;
        }
        if ql <= sl && sr <= qr {
            return self.v[pos];
        }

        let sm = (sl + sr) / 2;
        let lv = self._max(ql, qr, sl, sm, pos * 2);
        let rv = self._max(ql, qr, sm, sr, pos * 2 + 1);
        lv.max(rv)
    }
}

#[derive(Debug, Clone)]
pub struct RMinQ {
    v: Vec<i64>,
}

impl Default for RMinQ {
    fn default() -> Self {
        RMinQ::new()
    }
}

impl RMinQ {
    const SEQ_LEN: usize = 1<<19;
    const UNIT_VAL: i64 = 1_000_000_000;

    pub fn new() -> Self {
        Self {
            v: vec![RMinQ::UNIT_VAL; 2 * RMinQ::SEQ_LEN],
        }
    }

    /// Add `value` to i-th element.
    /// 0-origin.
    pub fn update(&mut self, mut index: usize, value: i64) {
        index += RMinQ::SEQ_LEN;
        self.v[index] = value;

        while index > 0 {
            index /= 2;
            self.v[index] = self.v[index * 2].min(self.v[index * 2 + 1]);
        }
    }

    /// Sum values on `[l, r)`. Note that it is a half-open interval.
    /// 0-origin.
    pub fn min(&self, l: usize, r: usize) -> i64 {
        self._min(l, r, 0, RMinQ::SEQ_LEN, 1)
    }

    fn _min(&self, ql: usize, qr: usize, sl: usize, sr: usize, pos: usize) -> i64 {
        if qr <= sl || sr <= ql {
            return RMinQ::UNIT_VAL;
        }
        if ql <= sl && sr <= qr {
            return self.v[pos];
        }

        let sm = (sl + sr) / 2;
        let lv = self._min(ql, qr, sl, sm, pos * 2);
        let rv = self._min(ql, qr, sm, sr, pos * 2 + 1);
        lv.min(rv)
    }
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<i64> = scanner.vec(n);
    let mut b: Vec<i64> = vec![0; n];
    
    for i in 0..n {
        if a[i] == 1 {
            b[i] = - 1;
        }
        else {
            b[i] = 1;
        }
    }

    let mut c: Vec<i64> = vec![0; n];
    c[0] = b[0];

    for i in 1..n {
        c[i] = b[i] + c[i - 1];
    }

    let mut rmaxq = RMaxQ::new();
    let mut rminq = RMinQ::new();
    for i in 0..n {
        rmaxq.update(i, c[i]);
        rminq.update(i, c[i]);
    }

    let mut maxv = 0;
    let mut minv = 0;

    let mut cur = 0;
    for i in 0..n {
        let u = rmaxq.max(i, n);
        let l = rminq.min(i, n);
        maxv = maxv.max(u - cur);
        minv = minv.min(l - cur);
        cur += b[i];
    }

    println!("{}", maxv - minv + 1);
}

use std::io::Write;
pub fn flush() { std::io::stdout().flush().unwrap(); }
pub struct Scanner { stdin: std::io::Stdin, buffer: std::collections::VecDeque<String> }
impl Scanner {
    fn new() -> Self {
        Self { stdin: std::io::stdin(), buffer: std::collections::VecDeque::new() }
    }

    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            line.split_whitespace().for_each(|x| self.buffer.push_back(String::from(x)));
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}
