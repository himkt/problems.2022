use proconio::input;


fn main() {
    input! {
        n: usize,
        a: usize,
        x: usize,
        y: usize,
    }

    if n > a {
        println!("{}", a * x + (n - a) * y);
    }
    else {
        println!("{}", n * x);
    }
}
