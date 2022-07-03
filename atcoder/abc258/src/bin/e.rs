pub fn lower_bound(range: std::ops::Range<usize>, prop: &dyn Fn(usize) -> bool) -> usize {
    if prop(range.start) {
        return range.start;
    }

    let mut ng = range.start;
    let mut ok = range.end;

    while ok - ng > 1 {
        let middle = ng + (ok - ng) / 2;
        match prop(middle) {
            true => ok = middle,
            false => ng = middle,
        }
    }

    ok
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let q: usize = scanner.cin();
    let x: usize = scanner.cin();
    let w: Vec<usize> = scanner.vec(n);
    let sum_weight = w.iter().sum::<usize>();

    let mut head_potatos = vec![];
    let mut head_members = HashSet::new();
    let mut num_potatos_in_pack = vec![];

    let mut cursor = 0;
    let last_potato;

    let mut w_double = vec![w.clone(), w.clone()].concat();
    let n2: usize = w_double.len();
    for i in 1..n2 {
        w_double[i] += w_double[i - 1];
    }

    loop {
        let i = cursor % n;

        if head_members.contains(&i) {
            last_potato = i;
            break;
        }

        let mut num_potatos = 0;
        let mut current_weight = 0;

        // x is larger than sum weight.
        if x >= sum_weight {
            let num_loops = x / sum_weight;
            num_potatos += n * num_loops;
            current_weight += sum_weight * num_loops;
        }

        // check if box is already full.
        if current_weight < x {
            let l = lower_bound(i..n2, &|p| {
                let buf = if i > 0 { w_double[i - 1] } else { 0 };
                let cumsum = w_double[p] - buf;
                current_weight + cumsum >= x
            });

            num_potatos += l - i + 1;
            cursor = l + 1;
        }
        else {
            cursor += 1;
        }

        head_potatos.push(i);
        head_members.insert(i);
        num_potatos_in_pack.push(num_potatos);
    }

    let mut cycle = 0;
    let mut in_cycle = false;
    let mut num_potatos_in_pack_before_cycle = vec![];
    let mut num_potatos_in_pack_after_cycle = vec![];
    let m: usize = num_potatos_in_pack.len();

    for i in 0..m {
        if head_potatos[i] == last_potato {
            in_cycle = true;
        }

        if in_cycle {
            num_potatos_in_pack_after_cycle.push(num_potatos_in_pack[i]);
            cycle += 1;
        }
        else {
            num_potatos_in_pack_before_cycle.push(num_potatos_in_pack[i]);
        }
    }

    for _ in 0..q {
        let k: usize = scanner.cin::<usize>() - 1;

        if k < num_potatos_in_pack_before_cycle.len() {
            println!("{}", num_potatos_in_pack_before_cycle[k]);
        }
        else {
            let k = k - num_potatos_in_pack_before_cycle.len();
            println!("{}", num_potatos_in_pack_after_cycle[k % cycle]);
        }
    }
}

use std::{io::Write, collections::HashSet}; pub fn flush() { std::io::stdout().flush().unwrap(); }
pub struct Scanner { buffer: std::collections::VecDeque<String>, buf: String }
impl Scanner {
    pub fn new() -> Self { Scanner { buffer: std::collections::VecDeque::new(), buf: String::new() } }
    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        if !self.buffer.is_empty() { return self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap(); }
        self.buf.truncate(0); std::io::stdin().read_line(&mut self.buf).ok();
        self.buf.to_owned().split_whitespace().for_each(|x| self.buffer.push_back(String::from(x)));
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> { (0..n).map(|_| self.cin()).collect() }
}
