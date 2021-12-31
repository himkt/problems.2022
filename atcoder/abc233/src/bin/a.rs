use proconio::input;


fn main() {
    input! {
        x: usize,
        y: usize,
    }

    if y <= x {
        println!("0");
    }
    else {
        println!("{}", (y - x + 9) / 10);
    }
}
