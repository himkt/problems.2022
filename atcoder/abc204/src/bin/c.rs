use proconio::input;
use std::collections::VecDeque;


fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut graph = Vec::<Vec::<usize>>::new();
    graph.resize(n, vec![]);

    for (ai, bi) in ab.into_iter() {
        graph[ai-1].push(bi-1);
    }

    let mut ans = 0;
    for start in 0..n {
        let mut queue = VecDeque::<usize>::new();
        queue.push_back(start);

        let mut visited = Vec::<bool>::new();
        visited.resize(n, false);

        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();
            if visited[cur] {
                continue
            }

            visited[cur] = true;
            for next in &graph[cur] {
                queue.push_back(*next);
            }
        }
        ans += visited
            .iter()
            .filter(|&&x| x)
            .count();
    }

    println!("{}", ans);
}
