use proconio::input;


fn solve(n: usize, k: usize, mut h: Vec<i32>) {
    h.sort();

    let mut dp = vec![0; n-k+1];

    for i in 0..n-k+1 {
        dp[i] = h[i+k-1] - h[i];
    }

    println!("{}", dp.iter().min().unwrap());
}


fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i32; n],
    }

    solve(n, k, h);
}
