use proconio::input;


fn main() {
    input! {
        a: usize,
        b: usize,
        m: usize,
        va: [i32; a],
        vb: [i32; b],
        x: [[i32; 3]; m]
    }

    let mut ans = va.iter().min().unwrap()
                  + vb.iter().min().unwrap();

    for xi in x {
        let ai = xi[0] as usize;
        let bi = xi[1] as usize;
        let c  = xi[2];
        ans = std::cmp::min(ans, va[ai-1] + vb[bi-1] - c);
    }

    println!("{}", ans);
}
