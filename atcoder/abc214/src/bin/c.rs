use proconio::input;


const INF: usize = 1001001001001;


fn main() {
    input! {
        n: usize,
        ss: [usize; n],
        ts: [usize; n],
    }

    let mut ans = vec![INF; n];
    ans[0] = ts[0];

    for t in 1..2*n {
        ans[t%n] = (ans[(t-1)%n] + ss[(t-1)%n]).min(ts[t%n]);
    }

    for ansi in ans {
        println!("{}", ansi);
    }
}
