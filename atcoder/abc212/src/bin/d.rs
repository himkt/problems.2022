use std::{cmp::Reverse, collections::BinaryHeap};


fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let q: usize = s.trim_end().parse::<usize>().unwrap();

    let mut queries = vec![];

    for _ in 0..q {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        queries.push(s.trim_end().to_owned());
    }

    let mut heap: BinaryHeap<Reverse<i128>> = BinaryHeap::new();
    let mut offset = 0;

    for query in queries {
        if query == "3" {
            let Reverse(v) = heap.pop().unwrap();
            println!("{}", v + offset)
        }
        else {
            let psp = query.split(' ').collect::<Vec<&str>>();
            let p = psp[0].chars().collect::<Vec<char>>()[0].to_digit(10).unwrap();
            let xi = psp[1].parse::<i128>().unwrap();

            if p == 1 {
                heap.push(Reverse(xi - offset));
            }
            if p == 2 {
                offset += xi;
            }
        }
    }
}
