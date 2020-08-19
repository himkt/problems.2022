use proconio::input;


fn solve(s: String, t: String) {
    let mut ans = 0;

    for i in 0..3 {
        if s[i..i+1] == t[i..i+1] {
            ans += 1
        }
    }

    println!("{}", ans);
}


fn main() {
    input! {
        s: String,
        t: String,
    }

    solve(s, t);
}
