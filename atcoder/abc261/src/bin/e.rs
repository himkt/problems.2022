fn o_and(a: usize, b: usize) -> usize {
    a & b
}

fn o_or(a: usize, b: usize) -> usize {
    a | b
}

fn o_xor(a: usize, b: usize) -> usize {
    a ^ b
}

fn gen_op(ti: usize) -> fn(usize, usize) -> usize {
    match ti {
        1 => o_and,
        2 => o_or,
        3 => o_xor,
        _ => panic!(),
    }
}

fn pick_nbit(n: usize, k: usize) -> usize {
    (n >> k) & 1
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut c: usize = scanner.cin();
    let mut t: Vec<usize> = vec![0; n];
    let mut a: Vec<usize> = vec![0; n];
    for i in 0..n {
        let ti: usize = scanner.cin();
        let ai: usize = scanner.cin();
        t[i] = ti;
        a[i] = ai;
    }

    let mut fss = vec![];
    for bit in 0..=30 {
        let mut fs = vec![(0, 1)];
        for i in 0..n {
            let (fi0, fi1) = fs[i];
            let pi = pick_nbit(a[i], bit);
            let op = gen_op(t[i]);
            fs.push((op(fi0, pi), op(fi1, pi)));
        }
        fss.push(fs);
    }

    for i in 1..=n {
        let mut vi = 0;
        for bit in 0..=30 {
            let pi = pick_nbit(c, bit);
            vi += match pi {
                0 => fss[bit][i].0<<bit,
                1 => fss[bit][i].1<<bit,
                _ => panic!(),
            };
        }
        println!("{}", vi);
        c = vi;
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
