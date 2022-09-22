fn get_center_size(h: usize, w: usize) -> (usize, usize) {
    /*

    (a) 2x5
    ..o..
    ..o..

    (b) 3x5
    .....
    ..o..
    .....

    (c) 2x1
    oo

    (d) 2x2
    oo
    oo

    (e) 3x3
    ...
    .o.
    ...

    (f) 4x4
    ....
    .oo.
    .oo.
    ....

    (h) 5x5
    .....
    .....
    ..o..
    .....
    .....

    (i) 6x5
    .....
    .....
    ..o..
    ..o..
    .....
    .....

    */

    let di = match h % 2 == 1 {
        true => vec![h / 2],
        false => vec![(h / 2) - 1, h / 2],
    };

    let dj = match w % 2 == 1 {
        true => vec![w / 2],
        false => vec![(w / 2) - 1, w / 2],
    };

    // 中心の座標
    if cfg!(debug_assertions) {
        let mut ret = vec![];
        for &dhi in di.iter() {
            for &dwj in dj.iter() {
                ret.push((dhi, dwj));
            }
        }
        println!("ret={:?}", ret);
    }

    (di.len(), dj.len())
}

fn consume_char(
    chars_with_two: &mut BinaryHeap<(usize, char)>,
    chars_with_fur: &mut BinaryHeap<(usize, char)>,
    chars_with_odd: &mut BinaryHeap<(usize, char)>,
    requires: usize,
) -> bool {

    if cfg!(debug_assertions) {
        println!("requires: {}", requires);
        println!("two={:?}, fur={:?}, odd={:?}", chars_with_two, chars_with_fur, chars_with_odd);
    }

    // odd のキューから取り出すしかない
    if requires % 2 == 1 {
        if let Some((count, char)) = chars_with_odd.pop() {
            let remaining = count - requires;

            if remaining > 0 {
                if remaining % 4 == 0 {
                    chars_with_fur.push((remaining, char));
                }
                else if remaining % 4 == 2 {
                    chars_with_two.push((remaining, char));
                }
            }
            return true;
        }
        else {
            return false;
        }
    }

    // requires が 2 ならまずは two のキューから取れないか試す
    if requires % 4 == 2 {
        if let Some((count, char)) = chars_with_two.pop() {
            let remaining = count - requires;
            if remaining > 0 {
                if remaining % 4 == 0 {
                    chars_with_fur.push((remaining, char));
                }
                else if remaining % 2 == 0 {
                    chars_with_two.push((remaining, char));
                }
            }
            return true;
        }
        else if let Some((count, char)) = chars_with_fur.pop() {
            let remaining = count - requires;
            if remaining > 0 {
                chars_with_two.push((remaining, char));
            }
            return true;
        }
        else {
            return false;
        }
    }

    // requires が 4 より大きいなら evn のキューから取れないか試す
    // evn が空だったら two のキューから取れないか試す (two は出現数が大きい順に文字が入っている)
    // count が require より大きい可能性があるので、その場合は別のキューに詰め直す
    if let Some((count, char)) = chars_with_fur.pop() {
        let remaining = count - requires;
        if remaining > 0 {
            if remaining % 4 == 0 {
                chars_with_fur.push((remaining, char));
            }
            else if remaining % 2 == 0 {
                chars_with_two.push((remaining, char));
            }
        }
        true
    }
    else if let Some((count, char)) = chars_with_two.pop() {
        if requires > count {
            return false;
        }

        let remaining = count - requires;
        if remaining > 0 {
            chars_with_two.push((remaining, char));
        }
        true
    }
    else {
        false
    }
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();

    let mut cnt = HashMap::new();
    for _ in 0..h {
        for ci in scanner.cin::<String>().chars() {
            *cnt.entry(ci).or_insert(0) += 1;
        }
    }

    let mut chars_with_two = BinaryHeap::new();
    let mut chars_with_fur = BinaryHeap::new();
    let mut chars_with_odd = BinaryHeap::new();

    if cfg!(debug_assertions) {
        println!("{:?}", cnt);
    }

    for (&char, &count) in cnt.iter() {
        if count % 4 == 2 {
            chars_with_two.push((count, char));
        }
        else if count % 4 == 0 {
            chars_with_fur.push((count, char));
        }
        else {
            chars_with_odd.push((count, char));
        }
    }

    let (height_center, width_center) = get_center_size(h, w);
    let center_volume = height_center * width_center;

    if cfg!(debug_assertions) {
        print!("[c] ");
    }

    if !consume_char(
        &mut chars_with_two,
        &mut chars_with_fur,
        &mut chars_with_odd,
        center_volume,
    ) {
        println!("No");
        return;
    }

    let mut height_current = height_center;
    let mut width_current = width_center;

    loop {
        let mut expanded_i = false;
        let mut expanded_j = false;

        if height_current < h {
            if cfg!(debug_assertions) {
                print!("[h] ");
            }

            if consume_char(
                &mut chars_with_two,
                &mut chars_with_fur,
                &mut chars_with_odd,
                2 * width_center,
            ) {
                height_current += 2;
                expanded_i = true;
            }
            else {
                println!("No");
                return;
            }
        }

        if width_current < w {
            if cfg!(debug_assertions) {
                print!("[w] ");
            }

            if consume_char(
                &mut chars_with_two,
                &mut chars_with_fur,
                &mut chars_with_odd,
                2 * height_center,
            ) {
                width_current += 2;
                expanded_j = true;
            }
            else {
                println!("No");
                return;
            }
        }

        let mut num_additional_slots = 0;
        if expanded_i && expanded_j {
            num_additional_slots += (height_current - height_center) / 2;
            num_additional_slots += (width_current - width_center) / 2;
            num_additional_slots -= 1;  // 重複除去
        }
        else if expanded_i {
            num_additional_slots = (width_current - width_center) / 2;
        }
        else if expanded_j {
            num_additional_slots = (height_current - height_center) / 2;
        }
        else {
            break;
        }

        for _ in 0..num_additional_slots {
            if cfg!(debug_assertions) {
                print!("[d] ");
            }

            if !consume_char(
                &mut chars_with_two,
                &mut chars_with_fur,
                &mut chars_with_odd,
                4)
            {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}

use std::{io::Write, collections::{BinaryHeap, HashMap}};
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
