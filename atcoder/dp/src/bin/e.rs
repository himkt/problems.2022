use proconio::input;


fn main () {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }

    let ws: Vec<usize> = wv.iter().map(|pair| pair.0).collect();
    let vs: Vec<usize> = wv.iter().map(|pair| pair.1).collect();

    const INF: usize = 1001001001;
    let mut dp = vec![vec![INF; n * 1000 + 1]; n + 1];
    dp[0][0] = 0;

    for i in 0..n {
        for j in 0..=(n * 1000) {
            if j < vs[i] {
                dp[i+1][j] = dp[i][j];
            }
            else {
                dp[i+1][j] = dp[i][j].min(dp[i][j-vs[i]] + ws[i]);
            }
        }
    }

    let mut ans = 0;
    for j in 0..=(n * 1000) {
        if dp[n][j] <= w {
            ans = j;
        }
    }

    println!("{}", ans);
}
