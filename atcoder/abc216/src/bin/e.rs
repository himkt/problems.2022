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

fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut k: usize = scanner.cin();
    let mut a: Vec<usize> = scanner.vec(n);
    a.sort_unstable_by_key(|&x| Reverse(x));

    let mut width = 1;
    let mut prev_height = a[0];

    let mut ans = 0;
    for i in 1..n {
        if a[i] != prev_height {
            let budget = width * (prev_height - a[i]);
            //println!("k={}, width={}, prev_height={}, a[i]={}, budget={}", k, width, prev_height, a[i], budget);
            if k <= budget {
                let d = prev_height - a[i];
                let n = lower_bound(0..(d + 1), &|x| x * width > k) - 1;
                let v = width * (n * (prev_height + prev_height - n + 1) / 2);
                //println!("[a] d={}, n={}, v={}", d, n, v);
                ans += v;
                k -= width * n;

                ans += (prev_height - n) * k;
                //println!("[a] prev_height={}, k={}", prev_height, k);
                println!("{}", ans);
                return;
            }
            else {
                k -= budget;
                let n = prev_height - a[i];
                let v = width * (n * (prev_height + a[i] + 1) / 2);
                //println!("[b] n={}, v={}, prev_height={}, a[i]={}", n, v, prev_height, a[i]);
                ans += v;
            }
        }
        width += 1;
        prev_height = a[i];
    }

    //println!("k={}, prev_height={}, width={}, ans={}", k, prev_height, width, ans);
    if k > 0 {
        let d = prev_height;
        let n = lower_bound(0..(d + 1), &|x| x * width > k) - 1;
        //println!("n={}", n);
        let v = width * (n * (prev_height + prev_height - n + 1) / 2);
        //println!("d={}, n={}, v={}", d, n, v);
        ans += v;
        k -= width * n;

        if k < width {
            //println!("k={}, prev_height={}", k, prev_height);
            ans += (prev_height - n) * k;
        }
    }

    println!("{}", ans);
}


use std::cmp::Reverse;
use std::collections::VecDeque;
use std::io::{self, Write};
use std::str::FromStr;

#[allow(dead_code)]
struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
#[allow(dead_code)]
impl Scanner {
    fn new() -> Self {
        Self {
            stdin: io::stdin(),
            buffer: VecDeque::new(),
        }
    }

    fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

#[allow(dead_code)]
fn flush() {
    std::io::stdout().flush().unwrap();
}
