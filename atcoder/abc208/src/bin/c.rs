use proconio::input;


fn main() {
    input! {
        n: usize,
        mut k: usize,
        a: [i32; n],
    }

    let mut ps = vec![];
    for (i, ai) in a.into_iter().enumerate() {
        ps.push((ai, i));
    }
    ps.sort_by(|a, b| a.0.cmp(&b.0));

    let base_ans = k / n;
    let mut ans = Vec::<usize>::new();
    ans.resize(n, base_ans);

    k -= base_ans * n;
    for i in 0..n {
        if i < k {
            ans[ps[i].1] += 1;
        }
    }

    for ansi in ans {
        println!("{}", ansi);
    }
}
