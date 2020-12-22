
use proconio::input;


fn solve(a: Vec<i64>) {
    let total: i64 = a.iter().sum();

    let mut cur = 0;
    let mut left = 0;

    for a_i in &a {
        if left + a_i > total / 2 {
            break;
        }

        left += a_i;
        cur += 1;
    }

    let right = total - left - a[cur];
    let ans = (std::cmp::max(left, right) - (std::cmp::min(left, right) + a[cur])).abs();
    println!("{}", ans);
}

fn main() {
    input! {
        n: i64,
        a: [i64; n],
    }

    solve(a);
}
