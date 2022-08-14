#[derive(Debug, Clone)]
pub struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    city_sizes: Vec<usize>,
    plant_sizes: Vec<usize>,
}

#[allow(clippy::needless_range_loop)]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            sizes: vec![1usize; n],
            city_sizes: vec![0; n],
            plant_sizes: vec![0; n],
        }
    }

    pub fn set_city(&mut self, u: usize) {
        self.city_sizes[u] = 1;
    }

    pub fn set_plant(&mut self, u: usize) {
        self.plant_sizes[u] = 1;
    }

    pub fn parent(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            x
        } else {
            self.parents[x] = self.parent(self.parents[x]);
            self.parents[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let mut px = self.parent(x);
        let mut py = self.parent(y);

        if px == py {
            return;
        }

        if self.sizes[px] < self.sizes[py] {
            std::mem::swap(&mut px, &mut py);
        }

        self.sizes[px] += self.sizes[py];
        self.city_sizes[px] += self.city_sizes[py];
        self.plant_sizes[px] += self.plant_sizes[py];
        self.parents[py] = px;
    }

    pub fn size(&mut self, x: usize) -> usize {
        let x = self.parent(x);
        self.sizes[x]
    }

    pub fn city_plant_size(&mut self, x: usize) -> (usize, usize) {
        let x = self.parent(x);
        (self.city_sizes[x], self.plant_sizes[x])
    }
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let e: usize = scanner.cin();
    let edges: Vec<(usize, usize)> = (0..e)
        .map(|_| {
            let u: usize = scanner.cin::<usize>() - 1;
            let v: usize = scanner.cin::<usize>() - 1;
            (u, v)
        })
        .collect();

    let q: usize = scanner.cin();
    let mut dead_edge_ids = HashSet::new();
    let mut dead_edge_ids_arr = vec![];
    for _ in 0..q {
        let x: usize = scanner.cin::<usize>() - 1;
        dead_edge_ids.insert(x);
        dead_edge_ids_arr.push(x);
    }

    let mut union_find = UnionFind::new(n + m);
    for i in 0..n {
        union_find.set_city(i);
    }
    for i in n..(n + m) {
        union_find.set_plant(i);
    }

    for i in 0..e {
        if dead_edge_ids.contains(&i) {
            continue;
        };
        let (u, v): (usize, usize) = edges[i];
        union_find.unite(u, v);
    }

    let mut num_connected_city = 0;
    let mut used = HashSet::new();
    for i in 0..(n + m) {
        let p = union_find.parent(i);
        if used.contains(&p) {
            continue;
        }
        used.insert(p);
        let (city_size, plant_size) = union_find.city_plant_size(p);
        if plant_size > 0 {
            num_connected_city += city_size;
        }
    }

    let mut history = vec![num_connected_city];
    for i in (0..q).rev() {
        let edge_id = dead_edge_ids_arr[i];
        let (u, v): (usize, usize) = edges[edge_id];
        let (city_size1, plant_size1): (usize, usize) = union_find.city_plant_size(u);
        let (city_size2, plant_size2): (usize, usize) = union_find.city_plant_size(v);
        if plant_size1 * plant_size2 == 0 && plant_size1 + plant_size2 > 0 {
            let new_city_size = if plant_size1 == 0 {
                city_size1
            } else {
                city_size2
            };
            num_connected_city += new_city_size;
        }
        union_find.unite(u, v);
        history.push(num_connected_city);
    }

    for v in history.iter().take(q).rev() {
        println!("{}", v);
    }
}

use std::{io::Write, collections::HashSet};
pub struct Scanner {
    buffer: std::collections::VecDeque<String>,
    buf: String,
}
#[allow(clippy::new_without_default)]
impl Scanner {
    pub fn new() -> Self {
        Scanner {
            buffer: std::collections::VecDeque::new(),
            buf: String::new(),
        }
    }
    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        if !self.buffer.is_empty() {
            return self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap();
        }
        self.buf.truncate(0);
        std::io::stdin().read_line(&mut self.buf).ok();
        self.buf
            .to_owned()
            .split_whitespace()
            .for_each(|x| self.buffer.push_back(String::from(x)));
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
    pub fn flush(&self) {
        std::io::stdout().flush().unwrap();
    }
}
