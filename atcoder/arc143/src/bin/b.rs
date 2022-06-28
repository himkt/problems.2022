const DIV: usize = 998244353;

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

pub fn modcom(fact: &Vec<usize>, finv: &Vec<usize>, n: usize, k: usize, div: usize) -> usize {
    fact[n] * (finv[k] * finv[n - k] % div) % div
}

pub fn modfac(n: usize, div: usize) -> usize {
    let mut ans = n;
    for i in (1..n).rev() {
        ans *= i;
        ans %= div;
    }
    ans % div
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    if n == 1 {
        println!("0");
        return;
    }

    let (fact, finv) = modcom_init(500 * 500, DIV);

    let mut cmb = 0;
    for x in 1..=(n * n) {
        if x - 1 < n - 1 {
            continue;
        }
        if n * n - x < n - 1 {
            continue;
        }

        let num_below = modcom(&fact, &finv, x - 1, n - 1, DIV);
        let num_upper = modcom(&fact, &finv, n * n - x, n - 1, DIV);
        cmb += num_below * num_upper;
        cmb %= DIV;
    }

    let f1 = modfac(n - 1, DIV);
    let f2 = modfac((n - 1) * (n - 1), DIV);

    // n^2 * (n - 1)! * (n - 1)! * ((n - 1)^2)! * cmb
    let mut complementary: usize = 1;
    complementary *= n;
    complementary %= DIV;
    complementary *= n;
    complementary %= DIV;
    complementary *= f1;
    complementary %= DIV;
    complementary *= f1;
    complementary %= DIV;
    complementary *= f2;
    complementary %= DIV;
    complementary *= cmb;
    complementary %= DIV;

    let mut ans = modfac(n * n, DIV) + DIV;
    ans -= complementary;
    ans %= DIV;
    println!("{}", ans);
}

use std::io::Write; pub fn flush() { std::io::stdout().flush().unwrap(); }
pub struct Scanner { buffer: std::collections::VecDeque<String>, buf: String }
impl Scanner {
    pub fn new() -> Self { Scanner { buffer: std::collections::VecDeque::new(), buf: String::new() } }
    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        if !self.buffer.is_empty() { return self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap() }
        self.buf.truncate(0); std::io::stdin().read_line(&mut self.buf).ok();
        self.buf.to_owned().split_whitespace().for_each(|x| self.buffer.push_back(String::from(x)));
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> { (0..n).map(|_| self.cin()).collect() }
}
