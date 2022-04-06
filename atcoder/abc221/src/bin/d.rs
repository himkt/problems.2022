#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut c: HashMap<usize, i64> = HashMap::new();

    for _ in 0..n {
        let a: usize = scanner.cin();
        let b: usize = scanner.cin();

        let s = a;
        let t = a + b;

        *c.entry(s).or_insert(0) += 1;
        *c.entry(t).or_insert(0) -= 1;
    }

    let mut keys: Vec<&usize> = c.keys().into_iter().collect();
    let num_keys: usize = keys.len();
    keys.sort();

    let mut cum: HashMap<usize, i64> = HashMap::new();

    let &first_key = keys[0];
    let &first_value = c.get(&first_key).unwrap();
    cum.insert(first_key, first_value);

    for idx in 1..num_keys {
        let &prev_key = keys[idx-1];
        let &crnt_key = keys[idx];

        let &prev_value = cum.get(&prev_key).unwrap();
        let &ci = c.get(&crnt_key).unwrap();

        cum.insert(crnt_key, prev_value + ci);
    }

    let mut keys: Vec<&usize> = cum.keys().into_iter().collect();
    keys.sort();

    let mut k1 = *keys[0];
    let mut ans: Vec<usize> = vec![0; n+1];

    for k2_idx in 1..keys.len() {
        let &k2 = keys[k2_idx];

        let &v1 = cum.get(&k1).unwrap();
        let &v2 = cum.get(&k2).unwrap();

        if v1 == v2 {
            continue;
        }

        ans[v1 as usize] += k2 - k1;
        k1 = k2;
    }

    let ans: Vec<_> = ans[1..]
        .iter()
        .map(|x| x.to_string())
        .collect();
    println!("{}", ans.join(" "));
}


use std::collections::{VecDeque, HashMap};
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
