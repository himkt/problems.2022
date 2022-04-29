pub fn lower_bound(range: std::ops::Range<usize>, prop: &dyn Fn(usize) -> bool) -> usize {
    if prop(range.start) {
        range.start
    }
    else {
        let mut ng = range.start;
        let mut ok = range.end;

        while ok - ng > 1 {
            let middle = ng + (ok - ng) / 2;

            if prop(middle) {
                ok = middle;
            }
            else {
                ng = middle;
            }
        }

        ok
    }
}


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let mut rs = vec![];
    let mut gs = vec![];
    let mut bs = vec![];

    for _ in 0..2*n {
        let a: i64 = scanner.cin();
        let c: char = scanner.cin();

        match c {
            'R' => rs.push(a),
            'G' => gs.push(a),
            'B' => bs.push(a),
            _ => panic!(),
        }
    }

    let all_even = [&rs, &gs, &bs]
        .iter()
        .all(|&xs| xs.len() % 2 == 0);

    if all_even {
        println!("0");
        return;
    }

    let (mut cs1, mut cs2, mut cs3) = if rs.len() % 2 == 0 {
        (gs, bs, rs)
    }
    else if gs.len() % 2 == 0 {
        (rs, bs, gs)
    }
    else if bs.len() % 2 == 0 {
        (rs, gs, bs)
    }
    else { panic!(); };

    cs1.sort_unstable();  // odd
    cs2.sort_unstable();  // odd
    cs3.sort_unstable();  // even

    let mut ans = 1_001_001_001_001_001_001;

    // (a) 奇数の色同士でペアを作る場合
    let u = cs2.len();
    for &c1 in cs1.iter() {
        let k = lower_bound(0..u, &|x| cs2[x] >= c1);

        if k == u {
            ans = ans.min((c1 - cs2[k-1]).abs());
            continue;
        }

        let mut cand = (c1 - cs2[k]).abs();

        if k > 0 {
            cand = cand.min((c1 - cs2[k-1]).abs());
        }
        if k + 1 < u {
            cand = cand.min((c1 - cs2[k+1]).abs());
        }

        ans = ans.min(cand);
    }

    if cs3.is_empty() {
        println!("{}", ans);
        return;
    }

    // (b) 奇数-偶数 の色同士でペアを２組つくる場合
    let u = cs3.len();

    let mut cand1s = vec![];
    for c1 in cs1 {
        let k = lower_bound(0..u, &|x| cs3[x] >= c1);

        if k == u {
            cand1s.push(((c1 - cs3[k-1]).abs(), k-1));
            continue;
        }

        cand1s.push(((c1 - cs3[k]).abs(), k));

        if k > 0 {
            cand1s.push(((c1 - cs3[k-1]).abs(), k-1));
        }
        if k + 1 < u {
            cand1s.push(((c1 - cs3[k+1]).abs(), k+1));
        }
    }

    let mut cand2s = vec![];
    for c2 in cs2 {
        let k = lower_bound(0..u, &|x| cs3[x] >= c2);

        if k == u {
            cand2s.push(((c2 - cs3[k-1]).abs(), k-1));
            continue;
        }

        cand2s.push(((c2 - cs3[k]).abs(), k));

        if k > 0 {
            cand2s.push(((c2 - cs3[k-1]).abs(), k-1));
        }
        if k + 1 < u {
            cand2s.push(((c2 - cs3[k+1]).abs(), k+1));
        }
    }

    cand1s.sort_unstable_by_key(|&(v, _)| v);
    cand2s.sort_unstable_by_key(|&(v, _)| v);

    // println!("cand1s: {:?}", cand1s);
    // println!("cand2s: {:?}", cand2s);

    let (cost1, cand1) = cand1s[0];
    for i in 0..cand2s.len() {
        let (cost2, cand2) = cand2s[i];
        if cand1 != cand2 {
            // println!("cand1={}, cand2={}, cost={}", cand1, cand2, cost1 + cost2);
            ans = ans.min(cost1 + cost2);
            break;
        }
    }

    let (cost2, cand2) = cand2s[0];
    for i in 0..cand1s.len() {
        let (cost1, cand1) = cand1s[i];
        if cand1 != cand2 {
            // println!("cand1={}, cand2={}, cost={}", cand1, cand2, cost1 + cost2);
            ans = ans.min(cost1 + cost2);
            break;
        }
    }

    println!("{}", ans);
}


use std::collections::VecDeque;
use std::io::{self, Write};
use std::str::FromStr;

pub struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
impl Scanner {
    fn new() -> Self {
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
    }

    pub fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

pub fn flush() {
    std::io::stdout().flush().unwrap();
}
