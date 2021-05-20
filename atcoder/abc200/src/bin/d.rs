use proconio::input;

pub struct Bitset<T: Copy> {
    curr: u32,
    array: Vec<T>,
    len: usize,
}

impl<T: Copy> Iterator for Bitset<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.curr >= 1<<self.len {
            return None;
        }

        let mut ret = Vec::<T>::new();
        let r_array = self.array.clone();
        for (i, ai) in r_array.iter().enumerate() {
            let patch = self.curr>>i & 1;
            if patch == 1 {
                ret.push(*ai);
            }
        }

        self.curr += 1;
        Some(ret)
    }
}

pub fn bitset<T: Copy>(a: Vec<T>) -> Bitset<T> {
    let len = a.len();
    Bitset { curr: 0, array: a, len }
}

fn solve(_: i64, a: Vec<i64>) {
    let mut arr: Vec<Vec<Vec<usize>>> = vec![vec![]; 200];
    let min_v = std::cmp::min(8, a.len());
    let arange: Vec<usize> = (0..min_v).collect();

    let bitset = bitset(arange);
    for candidate in bitset {
        let sum: i64 = candidate.iter().map(|idx| a[*idx] % 200).sum();
        arr[(sum % 200) as usize].push(candidate.to_vec());
    }

    for _arr in arr {
        if _arr.len() >= 2 {
            println!("Yes");
            let s0 = _arr[0].iter().map(|item| (1+item).to_string()).collect::<Vec<String>>();
            println!("{} {:}", _arr[0].len(), s0.join(" "));
            let s1 = _arr[1].iter().map(|item| (1+item).to_string()).collect::<Vec<String>>();
            println!("{} {:}", _arr[1].len(), s1.join(" "));
            return;
        }
    }

    println!("No");
}

fn main() {
    input! {
        n: i64,
        a: [i64; n],
    }
    solve(n, a);
}
