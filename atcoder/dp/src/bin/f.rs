#![allow(clippy::needless_range_loop,clippy::collapsible_else_if)]

use std::collections::VecDeque;
use proconio::input;


fn main() {
    input! {
        s_string: String,
        t_string: String,
    }

    let s: Vec<char> = s_string.chars().collect();
    let t: Vec<char> = t_string.chars().collect();

    let mut dp = vec![vec![0; t.len()+1]; s.len()+1];
    for i in 0..s.len() {
        for j in 0..t.len() {
            if s[i] == t[j] {
                dp[i+1][j+1] = dp[i][j] + 1;
                println!("{}, {}", i, j);
            }
            else {
                dp[i+1][j+1] = dp[i+1][j].max(dp[i][j+1]);
            }
        }
    }

    for row in dp.clone().into_iter() {
        println!("{:?}", row);
    }

    let mut i = s.len();
    let mut j = t.len();

    let mut ans = VecDeque::new();
    loop {
        if s[i-1] == t[j-1] {
            ans.push_front(s[i-1]);
            i -= 1;
            j -= 1;
        }

        else {
            if dp[i][j] == dp[i-1][j] {
                i -= 1;
            }
            else {
                j -= 1;
            }
        }

        if i == 0 || j == 0 {
            break;
        }
    }

    println!("{}", ans.iter().collect::<String>());
}
