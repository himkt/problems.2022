use proconio::input;


fn main() {
    input! {
        a: usize,
        b: usize,
        m: usize,
        va: [i32; a],
        vb: [i32; b],
        x: [(usize, usize, i32); m]
    }

    let mut ans = va.iter().min().unwrap() + vb.iter().min().unwrap();
    for (ai, bi, c) in x {
        ans = std::cmp::min(ans, va[ai-1] + vb[bi-1] - c);
    }

    println!("{}", ans);
}
