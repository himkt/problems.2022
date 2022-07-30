fn sign(i: usize) -> i64 {
    if i % 2 == 0 {
        return 1;
    }
    -1
}

#[allow(clippy::needless_range_loop)]
fn test(n: usize, a1: &[usize], a2: &[usize]) -> usize {
    let n_i64 = n as i64;
    let c: i64 = n as i64 / 2;

    let mut arr: Vec<usize> = vec![0; n];
    arr[c as usize] = a1[0];

    for i in 1..n {
        let i_i64 = i as i64;
        let x = (i_i64 + 1) / 2;
        let d = sign(i) * (2 * x - 1);

        if c + d < 0 || c + d > n_i64 - 1 {
            break;
        }

        let ni = (c + d) as usize;
        arr[ni] = a2[i - 1];
    }

    for i in 1..n {
        let i_i64 = i as i64;
        let x = (i_i64 + 1) / 2;
        let d = sign(i) * 2 * x;

        if c + d < 0 || c + d > n_i64 - 1 {
            break;
        }

        let ni = (c + d) as usize;
        arr[ni] = a1[i];
    }

    let mut ans = 0;
    for i in 1..n {
        ans += arr[i-1].max(arr[i]) - arr[i-1].min(arr[i]);
    }

    ans
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut a_asc: Vec<usize> = (0..n)
        .map(|_| scanner.cin::<usize>())
        .collect();
    a_asc.sort_unstable();

    let mut a_dsc: Vec<usize> = a_asc.to_vec();
    a_dsc.reverse();

    let ans = test(n, &a_asc, &a_dsc);
    let ans = ans.max(test(n, &a_dsc, &a_asc));
    println!("{}", ans);
}

use std::io::Write; pub fn flush() { std::io::stdout().flush().unwrap(); }
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
