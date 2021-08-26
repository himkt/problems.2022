use proconio::input;


fn solve(x: usize, a: Vec<usize>) {
    let mut sum: usize = 0;
    for (i, xi) in a.into_iter().enumerate() {
        if (i+1) % 2 == 0 {
            sum += xi - 1;
        }
        else {
            sum += xi;
        }
    }
    if sum > x {
        println!("No");
    }
    else {
        println!("Yes");
    }
}


fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    solve(x, a);
}
