use proconio::input;


fn reach(source: (&i64, &i64), target: (&i64, &i64), budget: i64) -> bool {
    let dist = (source.0 - target.0).abs() + (source.1 - target.1).abs();
    // println!("dist: {}, budget: {}", dist, budget);
    return dist <= budget && ((budget % 2 == 0) == (dist % 2 == 0));
}


fn solve(items: Vec<(i64, i64, i64)>) {
    let mut ct = 0;
    let mut cx = 0;
    let mut cy = 0;

    for (t, x, y) in &items {
        if !reach((&cx, &cy), (x, y), t - ct) {
            println!("No");
            return;
        }

        cx = *x;
        cy = *y;
        ct = *t;
    }

    println!("Yes");
}


fn main() {
    input! {
        n: i64,
        items: [(i64, i64, i64); n],
    }

    solve(items);
}
