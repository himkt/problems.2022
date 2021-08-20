use std::collections::BinaryHeap;

use proconio::input;


const INF: usize = 1001001001;


#[derive(Debug,Clone)]
pub struct Dijkstra {
    graph: Vec<Vec<(usize, usize)>>,
}

impl Dijkstra {
    pub fn new(graph: Vec<Vec<(usize, usize)>>) -> Self {
        Self {
            graph,
        }
    }

    pub fn search(&mut self, src: usize, limit: usize) -> Vec<usize> {
        let mut dist = vec![INF; self.graph.len()];
        dist[src] = 0;

        let mut queue = BinaryHeap::new();
        queue.push((std::cmp::Reverse(0), src));

        while let Some((current_cost, current_v)) = queue.pop() {
            if dist[current_v] < current_cost.0 {
                continue;
            }

            for (v, cost) in self.graph[current_v].clone() {
                if v > 1 && v - 1 > limit {
                    continue;
                }

                if dist[v] > current_cost.0 + cost {
                    dist[v] = current_cost.0 + cost;
                    queue.push((std::cmp::Reverse(dist[v]), v));
                }
            }
        }

        dist
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize, usize); m],
    }

    let mut original_dist = Vec::<Vec::<usize>>::new();
    original_dist.resize(n, vec![INF; n]);
    for (s, t, c) in edges.clone() {
        original_dist[s-1][t-1] = c;
    }

    let mut graph = Vec::<Vec::<(usize, usize)>>::new();
    graph.resize(n, vec![]);

    for (s, t, c) in edges.into_iter() {
        graph[s-1].push((t-1, c));
    }

    let mut ans = 0;
    let mut dijkstra = Dijkstra::new(graph);

    for k in 0..n {
        println!("k={}", k);
        for s in 0..n {
            let dist = dijkstra.search(s, k);

            let mut total_dist = 0;
            for t in 0..n {
                let ppp = dist[t];
                let ppk = original_dist[s][t];
                if ppp.min(ppk) != INF {
                    total_dist += ppp.min(ppk);
                }
                println!("{}->{}: {}", s, t, ppp.min(ppk));
            }

            ans += total_dist;
        }
    }

    println!("{}", ans);
}
