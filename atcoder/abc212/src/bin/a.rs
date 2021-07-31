use proconio::input;


fn solve() {
}


fn main() {
    input! {
        a: usize,
        b: usize,
    }

    if a > 0 && b > 0 {
        println!("Alloy");
    }
    else if a == 0 {
        println!("Silver");
    }
    else if b == 0 {
        println!("Gold");
    }
}
