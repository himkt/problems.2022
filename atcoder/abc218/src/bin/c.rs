#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let mut s: Vec<Vec<char>> = vec![vec!['-'; n]; n];
    let mut t: Vec<Vec<char>> = vec![vec!['-'; n]; n];

    for i in 0..n {
        let cs: String = scanner.cin();
        let cs: Vec<char> = cs.chars().collect();
        s[i] = cs;
    }
    let mut s_xmax = 0;
    let mut s_xmin = n;
    let mut s_ymax = 0;
    let mut s_ymin = n;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '#' {
                s_xmax = s_xmax.max(i);
                s_xmin = s_xmin.min(i);

                s_ymax = s_ymax.max(j);
                s_ymin = s_ymin.min(j);
            }
        }
    }

    for i in 0..n {
        let cs: String = scanner.cin();
        let cs: Vec<char> = cs.chars().collect();
        t[i] = cs;
    }
    let mut t_xmax = 0;
    let mut t_xmin = n;
    let mut t_ymax = 0;
    let mut t_ymin = n;
    for i in 0..n {
        for j in 0..n {
            if t[i][j] == '#' {
                t_xmax = t_xmax.max(i);
                t_xmin = t_xmin.min(i);

                t_ymax = t_ymax.max(j);
                t_ymin = t_ymin.min(j);
            }
        }
    }

    let s_xwidth = s_xmax - s_xmin + 1;
    let s_ywidth = s_ymax - s_ymin + 1;
    let t_xwidth = t_xmax - t_xmin + 1;
    let t_ywidth = t_ymax - t_ymin + 1;

    // println!("s: {:?}", s);
    // println!("t: {:?}", t);
    // println!("s: ({}, {}), ({}, {})", s_xmin, s_ymin, s_xmax, s_ymax);
    // println!("t: ({}, {}), ({}, {})", t_xmin, t_ymin, t_xmax, t_ymax);

    let s_nx = s_xwidth;
    let s_ny = s_ywidth;

    let mut s_minimum = vec![vec!['-'; s_ny]; s_nx];
    for (i, si) in (s_xmin..=s_xmax).enumerate() {
        for (j, sj) in (s_ymin..=s_ymax).enumerate() {
            s_minimum[i][j] = s[si][sj];
        }
    }

    let t_nx = t_xwidth;
    let t_ny = t_ywidth;

    let mut t_minimum = vec![vec!['-'; t_ny]; t_nx];
    for (i, ti) in (t_xmin..=t_xmax).enumerate() {
        for (j, tj) in (t_ymin..=t_ymax).enumerate() {
            t_minimum[i][j] = t[ti][tj];
        }
    }

    // println!("s:\n");
    // view(&s_minimum);
    // println!("\n====");
    // println!("t:\n");
    // view(&t_minimum);
    // println!("\n====");

    // println!("rot90(t):\n");
    // view(&rot90(&t_minimum));
    // println!("\n====");
    // println!("rot180(t):\n");
    // view(&rot90(&rot90(&t_minimum)));
    // println!("\n====");
    // println!("rot270(t):");
    // view(&rot90(&rot90(&rot90(&t_minimum))));


    let t_rot90 = rot90(&t_minimum);
    let t_rot180 = rot90(&t_rot90);
    let t_rot270 = rot90(&t_rot180);

    if s_minimum == t_minimum || s_minimum == t_rot90 || s_minimum == t_rot180 || s_minimum == t_rot270 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}


#[allow(clippy::needless_range_loop)]
fn rot90(t: &[Vec<char>]) -> Vec<Vec<char>> {
    let x = t.len();
    let y = t[0].len();

    let mut nt = vec![vec!['.'; x]; y];

    for i in 0..y {
        for j in 0..x {
            nt[i][j] = t[x-j-1][i];
        }
    }

    nt
}


#[allow(clippy::needless_range_loop)]
fn view(t: &[Vec<char>]) {
    for i in 0..t.len() {
        let s: String = t[i].iter().collect();
        println!("{}", s);
    }
}


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
