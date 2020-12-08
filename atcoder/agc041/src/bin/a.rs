use proconio::input;


fn solve(n: i128, a: i128, b: i128) {
    let da = a - 1;
    let db = n - b;
    let dab = b - a;

    let mut ans = (dab + 1) / 2;

    if dab % 2 != 0 {
        ans += std::cmp::min(da, db);
    }

    println!("{}", ans);
}


fn main() {
    input! {
        n: i128,
        a: i128,
        b: i128,
    }

    solve(n, a, b);
}
