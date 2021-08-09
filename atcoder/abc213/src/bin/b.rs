use proconio::input;


fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut vs = vec![];
    for (i, &ai) in a.iter().enumerate() {
        vs.push((ai, i+1));
    }

    vs.sort_unstable();
    println!("{:?}", vs[vs.len()-2].1);
}
