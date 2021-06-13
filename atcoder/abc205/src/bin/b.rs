use proconio::input;


fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }

    a.sort_unstable();

    let mut i = 1;
    for ai in a.into_iter() {
        if i != ai {
            println!("No");
            return;
        }
        i += 1;
    }

    println!("Yes");
}
