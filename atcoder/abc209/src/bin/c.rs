use proconio::input;


const INF: usize = 1_000_000_007;


fn main() {
    input! {
        n: usize,
        mut c: [usize; n],
    }

    c.sort_unstable();

    let mut ans = 1;
    for (i, ci) in c.into_iter().enumerate() {
        ans *= ci - i;
        ans %= INF;
    }

    ans %= INF;
    println!("{}", ans);
}
