#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let k: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut a_init = a.iter()
        .take(m)
        .cloned()
        .enumerate()
        .map(|(i, ai)| (ai, i))
        .collect::<Vec<(usize, usize)>>();
    a_init.sort_unstable();

    let mut st_1st = BTreeSet::new();
    let mut st_2nd = BTreeSet::new();
    let mut tot = 0;

    for i in 0..k {
        st_1st.insert(a_init[i]);
        tot += a_init[i].0;
    }
    for i in k..m {
        st_2nd.insert(a_init[i]);
    }

    debug!("1st: {:?}, 2nd: {:?}", st_1st, st_2nd);
    println!("{}", tot);

    for (i, j) in (m..n).enumerate() {
        let qot = (a[i], i);
        let qin = (a[j], j);

        if st_1st.contains(&qot) {
            st_1st.remove(&qot);
            tot -= qot.0;

            if let Some(&qtop) = st_2nd.iter().next() {
                if qin.0 < qtop.0 {
                    st_1st.insert(qin);
                    tot += qin.0;
                }
                else {
                    st_1st.insert(qtop);
                    st_2nd.remove(&qtop);
                    st_2nd.insert(qin);
                    tot += qtop.0;
                }
            }
            else {
                st_1st.insert(qin);
                tot += qin.0;
            }
        }
        else {
            st_2nd.remove(&qot);

            let &qtail = st_1st.iter().rev().next().unwrap();
            if qin.0 < qtail.0 {
                st_1st.remove(&qtail);
                tot -= qtail.0;
                st_1st.insert(qin);
                tot += qin.0;
                st_2nd.insert(qtail);
            }
            else {
                st_2nd.insert(qin);
            }
        }

        debug!("1st: {:?}, 2nd: {:?}", st_1st, st_2nd);
        println!("{}", tot);
    }
}

use std::{io::Write, collections::BTreeSet};
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
