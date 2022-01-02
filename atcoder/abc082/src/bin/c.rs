use std::collections::HashMap;

use proconio::input;


fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map: HashMap<usize, usize> = HashMap::new();

    for ai in a {
        *map.entry(ai).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (k, v) in map {
        match k > v {
            true => ans += v,
            false => ans += v - k,
        }
    }

    println!("{}", ans);
}
