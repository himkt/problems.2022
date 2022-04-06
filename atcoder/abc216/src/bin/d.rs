#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let _: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut a: Vec<Vec<usize>> = vec![vec![]; m];
    for i in 0..m {
        let k: usize = scanner.cin();
        let ai: Vec<usize> = scanner.vec(k);
        a[i] = ai;
    }

    let mut position_idxs = vec![0; m];
    let mut visible_idxs_by_color: HashMap::<usize, Vec<usize>> = HashMap::new();
    for i in 0..m {
        let c = a[i][0];
        visible_idxs_by_color.entry(c).or_insert_with(Vec::new).push(i);
    }

    let mut selectable_colors = VecDeque::new();
    for (&color, idxs) in visible_idxs_by_color.iter() {
        if idxs.len() == 2 {
            selectable_colors.push_back(color);
        }
    }

    while let Some(color) = selectable_colors.pop_front() {
        let idxs = visible_idxs_by_color
            .get(&color)
            .unwrap()
            .clone();  // move out reference

        for next_idx in idxs.clone() {
            if position_idxs[next_idx] < a[next_idx].len() - 1 {
                position_idxs[next_idx] += 1;
                let next_color = a[next_idx][position_idxs[next_idx]];

                let vs = visible_idxs_by_color
                    .entry(next_color)
                    .or_insert_with(Vec::new);

                vs.push(next_idx);
                if vs.len() == 2 {
                    selectable_colors.push_back(next_color);
                }
            }
        }

        visible_idxs_by_color.remove(&color);
    }

    if visible_idxs_by_color.is_empty() {
        println!("Yes");
    }
    else {
        println!("No");
    }
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
