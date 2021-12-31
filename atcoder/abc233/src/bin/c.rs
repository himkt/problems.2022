use std::collections::VecDeque;
use proconio::input;


fn main() {
    input! {
        n: usize,
        x: u128,
    }

    let mut vss = vec![];

    for _ in 0..n {
        input! {
            l: usize,
            mut vs: [u128; l],
        }
        vs.sort_unstable();
        vss.push(vs);
    }

    let mut queue = VecDeque::new();
    queue.push_back((0usize, 1u128));

    let mut ans = 0;
    while !queue.is_empty() {
        let current_l = queue.front().unwrap().0;
        let current_v = queue.front().unwrap().1;
        queue.pop_front();

        if current_l == vss.len() {
            if current_v == x {
                ans += 1;
            }
            continue;
        }

        for &next_v in &vss[current_l] {
            if current_v * next_v <= x {
                queue.push_back((current_l + 1, current_v * next_v));
            }
            else {
                break;
            }
        }
    }

    println!("{}", ans);
}
