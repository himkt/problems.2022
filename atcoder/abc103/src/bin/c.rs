use proconio::input;


fn solve(n: i64, a: Vec<i64>) {
    let mut ans: i64 = a.iter().sum();
    ans -= n;
    println!("{}", ans);
}


fn main() {
    input! {
        n: i64,
        a: [i64; n],
    }

    solve(n, a);
}
