use proconio::input;


fn solve(s: String) {
    let mut position = 0;

    for (i, si) in s.chars().enumerate() {
        if si == '1' {
            position = i;
            break;
        }
    }

    if position % 2 == 0 {
        println!("Takahashi");
    }
    else {
        println!("Aoki");
    }
}


fn main() {
    input! {
        _: usize,
        s: String,
    }

    solve(s);
}
