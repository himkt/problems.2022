const DIV: usize = 998244353;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let p: usize = scanner.cin();
    let denominator = modinv(100, DIV);
    let pr = (p * denominator) % DIV;
    let qr = ((100 - p) * denominator) % DIV;

    let mut dp = vec![0; n + 1];
    dp[1] = 1;

    for i in 2..=n {
        // dp[2] => (a) dp[0] から (b) dp[1] から
        dp[i] = 1 + pr * dp[i - 2] + qr * dp[i - 1];
        dp[i] %= DIV;
    }

    debug!("{:?}", dp);
    println!("{}", dp[n]);
}

use std::{io::Write};
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

#[macro_export]
macro_rules! debug {
    () => {
        #[cfg(debug_assertions)]
        println!();
    };
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}

pub fn modcom(n: usize, k: usize, div: usize) -> usize {
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

    fact[n] * (finv[k] * finv[n - k] % div) % div
}

pub fn modpow(mut a: usize, mut n: usize, m: usize) -> usize {
    let mut ans = 1;

    while n > 0 {
        if n & 1 == 1 {
            ans = ans * a % m;
        }

        a = a * a % m;
        n >>= 1;
    }

    ans
}

pub fn phi(mut n: usize) -> usize {
    let mut res = n;

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            res = res / i * (i - 1);
            while n % i == 0 {
                n /= i;
            }
        }
        i += 1;
    }

    if n != 1 {
        res = res / n * (n - 1);
    }

    res
}

pub fn modinv(a: usize, p: usize) -> usize {
    let m = phi(p);
    modpow(a, m - 1, p)
}
