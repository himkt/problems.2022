#[derive(Debug,Clone)]
pub struct RAQ {
    v: Vec<i64>,
}

impl RAQ {
    const SEQ_LEN: usize = 1<<18;
}

impl Default for RAQ {
    fn default() -> Self {
        RAQ::new()
    }
}

impl RAQ {
    pub fn new() -> Self {
        Self {
            v: vec![0; 2*RAQ::SEQ_LEN]
        }
    }

    pub fn get(&mut self, mut index: usize) -> i64 {
        index += RAQ::SEQ_LEN - 1;

        let mut sum = 0;
        sum += self.v[index];

        loop {
            index /= 2;
            if index == 0 { break; }
            sum += self.v[index];
        }

        sum
    }

    pub fn add(&mut self, mut l: usize, mut r: usize, value: i64) {
        l += RAQ::SEQ_LEN - 1;
        r += RAQ::SEQ_LEN;

        while l < r {
            if l % 2 == 1 {
                self.v[l] += value;
                l += 1;
            }
            l /= 2;

            if r % 2 == 1 {
                self.v[r-1] += value;
                r -= 1;
            }
            r /= 2;
        }
    }

    pub fn from(v: Vec<i64>) -> Self {
        let mut rsq = RAQ::new();
        for (index, value) in (0..v.len()).zip(v.into_iter()) {
            let idx = RAQ::SEQ_LEN + index;
            rsq.v[idx] = value;
        }

        rsq
    }
}


#[allow(clippy::many_single_char_names)]
fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s = s.trim_end().to_owned();

    let mut ws = s.split_whitespace();
    let _: usize = ws.next().unwrap().parse().unwrap();
    let q: usize = ws.next().unwrap().parse().unwrap();

    let mut rsq = RAQ::new();
    for _ in 0..q {
        s.clear();
        std::io::stdin().read_line(&mut s).unwrap();
        s = s.trim_end().to_owned();

        let mut ws = s.split_whitespace();
        let c: usize = ws.next().unwrap().parse().unwrap();
        let i: usize = ws.next().unwrap().parse().unwrap();

        if c == 0 {
            let r: usize = ws.next().unwrap().parse().unwrap();
            let v: i64 = ws.next().unwrap().parse().unwrap();
            rsq.add(i, r, v);
        }
        else {
            println!("{}", rsq.get(i));
        }
    }
}
