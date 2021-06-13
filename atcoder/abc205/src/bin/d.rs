use proconio::input;
use std::{cmp::Ordering, collections::HashSet};


pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}


fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        q: [i64; q],
    }

    let mut ah = HashSet::<i64>::new();
    for ai in a.clone().into_iter() {
        ah.insert(ai);
    }

    for qi in q.iter() {
        let mut low = 1e0 as i64;
        let mut upp = 2e18 as i64;

        while low != upp {
            // println!("{}, {}", low, upp);
            let mid = (low+upp) / 2;
            let a_min_index = a.lower_bound(&mid) as i64;
            // println!("a_min_index: {}", a_min_index);

            if &(mid - a_min_index) == qi && !ah.contains(&mid) {
                low = mid;
                break;
            }
            else if &(mid - a_min_index) > qi {
                upp = mid;
            }
            else {
                low = mid;
            }
        }
        println!("{}", low);
        // println!("q: {} => {}, {}", qi, low, upp);
    }
}
