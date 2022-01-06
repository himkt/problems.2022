use std::collections::{BinaryHeap, HashSet};

const DIV: usize = 200;


#[allow(clippy::many_single_char_names)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);
    let a: Vec<_> = a.iter().map(|x| x % DIV).collect();

    let mut dp: Vec<Vec<usize>> = vec![vec![0; DIV]; n+1];
    for i in 1..=n {
        dp[i][a[i-1]] = 1;
    }

    for i in 0..n {
        for j in 0..DIV {
            let k = (j + a[i]) % DIV;
            dp[i+1][j] += dp[i][j];  // i 番目を使わない
            dp[i+1][k] += dp[i][j];  // i 番目を使う

            // 2 例だけわかれば十分なので 2 との min を取る
            dp[i+1][j] = dp[i+1][j].min(2);
            dp[i+1][k] = dp[i+1][k].min(2);
        }
    }

    let ans_j = (&dp[n])
        .iter()
        .enumerate()
        .filter(|&(_, x)| x >= &2)
        .map(|(i, _)| i)
        .next();

    if let Some(j) = ans_j {
        let mut queue = BinaryHeap::new();
        queue.push((vec![], n, j));

        let mut ans_set: HashSet<Vec<usize>> = HashSet::new();

        while !queue.is_empty() && ans_set.len() < 2 {
            let (mut path, i, j) = queue.pop().unwrap();
            let k = (DIV + j - a[i-1]) % DIV;

            if j == a[i-1] {
                let mut path = path.to_owned();
                path.push(i);
                ans_set.insert(path.into_iter().rev().collect());
            }

            if dp[i-1][j] > 0 {
                queue.push((path.to_owned(), i-1, j));
            }

            if dp[i-1][k] > 0 {
                path.push(i);
                queue.push((path, i-1, k));
            }
        }

        for (i, &ai) in a.iter().enumerate() {
            if ai == j && !ans_set.contains(&vec![i+1]) {
                ans_set.insert(vec![i+1]);
            }
        }

        let mut ans_iter = ans_set.into_iter();
        let b: Vec<_> = ans_iter.next().unwrap().iter().map(|x| x.to_string()).collect();
        let c: Vec<_> = ans_iter.next().unwrap().iter().map(|x| x.to_string()).collect();
        println!("Yes");
        println!("{} {}", b.len(), b.join(" "));
        println!("{} {}", c.len(), c.join(" "));
    }
    else {
        println!("No");
    }
}


use std::io::{self, Write};
use std::str::FromStr;
use std::collections::VecDeque;

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

    fn usize1(&mut self) -> usize {
        self.cin::<usize>() - 1
    }

    fn chars(&mut self) -> Vec<char> {
        self.cin::<String>().chars().collect()
    }

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

#[allow(dead_code)]
fn flush() {
    std::io::stdout().flush().unwrap();
}
