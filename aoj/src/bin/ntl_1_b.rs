const DIV: usize = 1_000_000_007;


pub fn modpow(a: usize, b: usize, m: usize) -> usize {
    let mut ans = 1;
    let mut p = a;

    for i in 0..30 {
        if b & (1<<i) != 0 {
            ans *= p;
            ans %= m;
        }

        p *= p;
        p %= m;
    }

    ans
}


fn main () {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer = buffer.trim_end().to_owned();

    let mut ws = buffer.split_whitespace();
    let m: usize = ws.next().unwrap().parse().unwrap();
    let n: usize = ws.next().unwrap().parse().unwrap();

    println!("{}", modpow(m, n, DIV));
}
