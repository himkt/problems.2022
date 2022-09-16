const DIV: usize = 998244353;

#[derive(Debug, Clone)]
pub struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

#[allow(clippy::needless_range_loop)]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            sizes: vec![1usize; n],
        }
    }

    pub fn parent(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            x
        } else {
            self.parents[x] = self.parent(self.parents[x]);
            self.parents[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let mut px = self.parent(x);
        let mut py = self.parent(y);

        if px == py {
            return;
        }

        if self.sizes[px] < self.sizes[py] {
            std::mem::swap(&mut px, &mut py);
        }

        self.sizes[px] += self.sizes[py];
        self.parents[py] = px;
    }

    pub fn size(&mut self, x: usize) -> usize {
        let x = self.parent(x);
        self.sizes[x]
    }
}

pub fn modcom_init(n: usize, div: usize) -> (Vec<usize>, Vec<usize>) {
    let mut fact = Vec::<usize>::new();
    let mut inv = Vec::<usize>::new();
    let mut finv = Vec::<usize>::new();

    let upper = n + 1;

    fact.resize(upper, 0);
    inv.resize(upper, 0);
    finv.resize(upper, 0);

    fact[0] = 1;
    fact[1] = 1;

    finv[0] = 1;
    finv[1] = 1;

    inv[1] = 1;

    for i in 2..upper {
        fact[i] = fact[i - 1] * i % div;
        inv[i] = div - inv[div % i] * (div / i) % div;
        finv[i] = finv[i - 1] * inv[i] % div;
    }

    (fact, finv)
}

pub fn modcom(n: usize, k: usize, div: usize, fact: &[usize], finv: &[usize]) -> usize {
    fact[n] * (finv[k] * finv[n - k] % div) % div
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let mut graph: Vec<usize> = (0..n).collect();
    for i in 0..n {
        let fi: usize = scanner.cin::<usize>() - 1;
        graph[i] = fi;
    }

    let mut num_cycles = 0;
    let mut union_find = UnionFind::new(n);
    for u in 0..n {
        let v = graph[u];
        let pu = union_find.parent(u);
        let pv = union_find.parent(v);

        if pu == pv {
            num_cycles += 1;
        }

        union_find.unite(u, v);
    }

    let (fact, finv) = modcom_init(n, DIV);
    let mut ans = 0;
    for k in 1..=num_cycles {
        ans += modcom(num_cycles, k, DIV, &fact, &finv);
        ans %= DIV;
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
