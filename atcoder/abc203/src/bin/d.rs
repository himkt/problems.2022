use proconio::input;


fn solve(n: usize, k: usize, a: &[Vec<i32>]) {
    let mut left = -1;
    let mut right = 1_000_000_000;

    let limit = (k * k / 2) + 1;
    let mut s = vec![vec![0; n+1]; n+1];

    while left + 1 < right {
        let mid = (left + right) / 2;

        for i in 0..n {
            for j in 0..n {
                s[i+1][j+1] = s[i+1][j] + s[i][j+1] - s[i][j];
                if a[i][j] > mid {
                    s[i+1][j+1] += 1;
                }
            }
        }

        if check(&s, n, k, limit) {
            right = mid;
        }
        else {
            left = mid;
        }
    }

    println!("{}", right);
}


fn check(s: &[Vec<usize>], n: usize, k: usize, limit: usize) -> bool {
    for i in 0..(n-k+1) {
        for j in 0..(n-k+1) {
            if s[i+k][j+k] + s[i][j] - s[i][j+k] - s[i+k][j] < limit {
                return true;
            }
        }
    }
    false
}


fn main() {
    input! {
        n: usize,
        k: usize,
        a: [[i32; n]; n],
    }

    solve(n, k, &a);
}
