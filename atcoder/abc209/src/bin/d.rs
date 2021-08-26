use proconio::input;
use std::collections::VecDeque;


const INF: usize = 1001001001;


#[derive(Debug,Clone)]
pub struct BFS {
    graph: Vec<Vec<usize>>,
    seen: Vec<bool>,
    dist: Vec<usize>,
}


impl BFS {
    pub fn new(graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        Self {
            graph,
            seen: vec![false; n],
            dist: vec![INF; n],
        }
    }

    pub fn search(&mut self, root: usize) {
        let mut queue = VecDeque::new();
        queue.push_back((root, 0));

        while !queue.is_empty() {
            let (cur, dist) = queue.pop_front().unwrap();
            if self.seen[cur] {
                continue;
            }

            self.seen[cur] = true;
            self.dist[cur] = self.dist[cur].min(dist);
            for &next in &self.graph[cur] {
                queue.push_back((next, self.dist[cur] + 1));
            }
        }
    }
}


fn main() {
    input! {
        n: usize,
        q: usize,
        edges: [(usize, usize); n-1],
        queries: [(usize, usize); q],
    }

    let mut graph = vec![vec![]; n];
    for (s, t) in edges {
        graph[s-1].push(t-1);
        graph[t-1].push(s-1);
    }

    let mut dist: Vec<Vec<usize>> = vec![vec![]; n];
    for s in 0..n {
        let mut bfs = BFS::new(graph.clone());
        bfs.search(s);
        dist[s] = bfs.dist;
    }

    for (s, t) in queries {
        if dist[s-1][t-1] % 2 == 0 {
            println!("Town");
        }
        else {
            println!("Road");
        }
    }
}
