use std::collections::HashMap;

use proconio::input;


fn count(cnt: &HashMap<usize, usize>) -> usize {
    let mut ans = 0;
    for v in cnt.values() {
        if v > &0 {
            ans += 1;
        }
    }
    ans
}


fn solve(n: usize, k: usize, c: Vec<usize>) -> usize {
    let mut cnt: HashMap<usize, usize> = HashMap::new();

    // setup
    for v in &c[0..k] {
        *cnt.entry(*v).or_insert(0) += 1;
    }

    let mut cur = count(&cnt);
    let mut ans = cur;

    for (i, j) in (k..n).enumerate() {

        // delete: c[i];
        *cnt.entry(c[i]).or_insert(0) -= 1;

        // if {a, b, b, c} -> {b, b, c}
        if cnt.get(&c[i]).unwrap() == &0 {
            cur -= 1;
        }

        // if c[j] is new item, prev would be true
        let mut prev = true;
        if let Some(v) = cnt.get(&c[j]) {
            if v > &0 {
                prev = false;
            }
        }

        // add: c[j];
        *cnt.entry(c[j]).or_insert(0) += 1;

        if prev && cnt.get(&c[j]).unwrap() == &1 {
            cur += 1;
        }

        ans = ans.max(cur);
    }

    ans
}


fn main() {
    input! {
        n: usize,
        k: usize,
        c: [usize; n],
    }

    println!("{}", solve(n, k, c));
}
