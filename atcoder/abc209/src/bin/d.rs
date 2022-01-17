use proconio::input;
use std::collections::VecDeque;


#[derive(Debug,Clone)]
pub struct BFS {
    graph: Vec<Vec<usize>>,
    seen: Vec<bool>,
    colors: Vec<usize>,
}


impl BFS {
    pub fn new(graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        Self {
            graph,
            seen: vec![false; n],
            colors: vec![0; n],
        }
    }

    pub fn search(&mut self, root: usize) {
        let mut queue = VecDeque::new();
        queue.push_back((root, 0));

        while !queue.is_empty() {
            let (cur, color) = queue.pop_front().unwrap();
            self.colors[cur] = color;

            if self.seen[cur] {
                continue;
            }

            self.seen[cur] = true;
            for &next in &self.graph[cur] {
                queue.push_back((next, (color + 1) % 2));
            }
        }
    }
}


#[allow(clippy::needless_range_loop)]
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

    let mut bfs = BFS::new(graph);
    bfs.search(0);

    for (c, d) in queries {
        if bfs.colors[c-1] == bfs.colors[d-1] {
            println!("Town");
        }
        else {
            println!("Road");
        }
    }
}
