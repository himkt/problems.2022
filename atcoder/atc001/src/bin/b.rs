use proconio::input;


#[derive(Debug,Clone)]
pub struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>
}

#[allow(clippy::needless_range_loop)]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            ranks: vec![1usize; n]
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            x
        }
        else {
            self.parents[x] = self.find(self.parents[x]);
            self.parents[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let mut px = self.find(x);
        let mut py = self.find(y);

        if px == py {
            return;
        }

        if self.ranks[px] > self.ranks[py] {
            std::mem::swap(&mut px, &mut py);
        }

        if self.ranks[px] == self.ranks[py] {
            self.ranks[py] += 1;
        }

        self.parents[px] = py;
    }
}


fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut uf = UnionFind::new(n);
    for _ in 0..q {
        input! {
            p: usize,
            a: usize,
            b: usize,
        }
        
        if p == 0 {
            uf.unite(a, b);
        }
        if p == 1 {
            if uf.find(a) == uf.find(b) {
                println!("Yes");
            }
            else {
                println!("No");
            }
        }
    }
}
