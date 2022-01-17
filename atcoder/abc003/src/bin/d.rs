use proconio::input;


const DIV: usize = 1_000_000_000 + 7;


fn combination(n: usize, k: usize, div: usize) -> usize {
    let mut fact = Vec::<usize>::new();
    let mut inv = Vec::<usize>::new();
    let mut finv = Vec::<usize>::new();

    let upper = n + 1;

    fact.resize(upper, 0);
    inv.resize(upper, 0);
    finv.resize(upper, 0);

    fact[0] = 1;
    fact[1] = 1;

    finv[0] = 1;
    finv[1] = 1;

    inv[1] = 1;

    for i in 2..upper {
        fact[i] = fact[i-1] * i % div;
        inv[i] = div - inv[div % i] * (div / i) % div;
        finv[i] = finv[i-1] * inv[i] % div;
    }

    fact[n] * (finv[k] * finv[n - k] % div) % div
}



fn solve(r: usize, c: usize, x: usize, y: usize, d: usize, l: usize) -> usize {
    let row = r - x + 1;
    let col = c - y + 1;
    (row * col * combination(d+l, d, DIV)) % DIV
}


fn main() {
    input! {
        r: usize,
        c: usize,
        x: usize,
        y: usize,
        d: usize,
        l: usize,
    }
    let ans = solve(r, c, x, y, d, l);
    println!("{}", ans);
}
