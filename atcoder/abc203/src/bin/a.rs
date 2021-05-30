use proconio::input;


fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }

    if a == b {
        println!("{}", c);
    }
    else if b == c {
        println!("{}", a);
    }
    else if c == a {
        println!("{}", b);
    }
    else {
        println!("0");
    }
}
