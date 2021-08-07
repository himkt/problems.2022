use proconio::input;


fn main () {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }

    let ws: Vec<usize> = wv.iter().map(|pair| pair.0).collect();
    let vs: Vec<usize> = wv.iter().map(|pair| pair.1).collect();

    let mut dp = vec![vec![0; w+1]; n+1];

    for i in 0..n {
        for j in 0..=w {
            if j < ws[i] {
                dp[i+1][j] = dp[i][j];
            }
            else {
                dp[i+1][j] = dp[i][j].max(dp[i][j-ws[i]] + vs[i]);
            }
        }
    }

    println!("{}", dp[n][w]);
}
