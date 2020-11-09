use proconio::input;


fn solve(s: String) {
    let m = 3;
    let n = s.len();

    let mut dp = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            dp[i][j] = 1;
        }
    }

    for i in 0..n {
        for j in 1..m {
            for k in 1..m {
                if i < j || i + k > n {
                    continue;
                }

                if s[i-j..i] != s[i..i+k] {
                    dp[i+k-1][k-1] = std::cmp::max(dp[i+k-1][k-1], dp[i-1][j-1]+1);
                }
            }
        }
    }

    println!("{}", dp[n-1].iter().max().unwrap());
}


fn main() {
    input! {
        s: String,
    }

    solve(s);
}
