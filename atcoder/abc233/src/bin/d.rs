use std::collections::HashMap;

use proconio::input;



#[allow(clippy::many_single_char_names)]
#[allow(clippy::needless_range_loop)]
fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let mut b = vec![0; n];
    b[0] = a[0];
    for i in 1..n {
        b[i] = b[i-1] + a[i];
    }

    let mut ans = 0;
    let mut map: HashMap<i64, i64> = HashMap::new();
    map.insert(0, 1);

    for i in 0..n {
        if let Some(&v) = map.get(&(b[i] - k)) {
            ans += v;
        }
        *map.entry(b[i]).or_insert(0) += 1;
    }

    println!("{}", ans);
}
