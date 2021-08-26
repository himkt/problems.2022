use proconio::input;


fn main() {
    input! {
        a: usize,
        b: usize,
    }

    if b > a {
        println!("{}", b - a + 1);
    }
    else {
        println!("0");
    }
}
