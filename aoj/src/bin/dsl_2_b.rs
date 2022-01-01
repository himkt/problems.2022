#[derive(Debug,Clone)]
pub struct RSQ {
    v: Vec<i64>,
}

impl RSQ {
    const SEQ_LEN: usize = 1<<18;
}

impl Default for RSQ {
    fn default() -> Self {
        RSQ::new()
    }
}

impl RSQ {
    pub fn new() -> Self {
        Self {
            v: vec![0; 2*RSQ::SEQ_LEN]
        }
    }

    pub fn add(&mut self, mut index: usize, value: i64) {
        index += RSQ::SEQ_LEN - 1;
        self.v[index] += value;

        loop {
            index /= 2;
            if index == 0 { break; }
            self.v[index] = self.v[index*2] + self.v[index*2 + 1];
        }
    }

    pub fn sum(&self, mut l: usize, mut r: usize) -> i64 {
        l += RSQ::SEQ_LEN - 1;
        r += RSQ::SEQ_LEN;

        let mut ans = 0;

        while l < r {
            if l % 2 == 1 {
                ans += self.v[l];
                l += 1;
            }
            l /= 2;

            if r % 2 == 1 {
                ans += self.v[r-1];
                r -= 1;
            }
            r /= 2;
        }

        ans
    }

    pub fn from(v: Vec<i64>) -> Self {
        let mut rsq = RSQ::new();
        for (index, value) in (0..v.len()).zip(v.into_iter()) {
            let idx = RSQ::SEQ_LEN + index;
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

    let mut rsq = RSQ::new();
    for _ in 0..q {
        s.clear();
        std::io::stdin().read_line(&mut s).unwrap();
        s = s.trim_end().to_owned();

        let mut ws = s.split_whitespace();
        let c: usize = ws.next().unwrap().parse().unwrap();
        let x: usize = ws.next().unwrap().parse().unwrap();
        let y: i64 = ws.next().unwrap().parse().unwrap();

        if c == 0 {
            rsq.add(x, y);
        }
        else {
            println!("{}", rsq.sum(x, y as usize));
        }
    }
}
