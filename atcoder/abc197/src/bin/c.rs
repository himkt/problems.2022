use proconio::input;


#[allow(clippy::needless_range_loop)]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans: usize = 1<<31;

    for i in 0..1<<(n-1) {
        let mut xored = 0;
        let mut  ored = a[0];

        for j in 1..n {
            let div = i>>(j-1) & 1;

            if div == 1 {
                xored ^= ored;
                ored = 0;  // initialize
            }

            ored |= a[j];
        }

        xored ^= ored;
        ans = ans.min(xored);
    }

    println!("{}", ans);
}
