use proconio::input;
use std::collections::HashSet;


fn main() {
    input! {
        a: [[i32; 3]; 3],
        n: i32,
        b: [i32; n],
    };

    let mut rs = Vec::new();
    let mut cs = Vec::new();

    let mut xs = vec![std::collections::HashSet::<i32>::new(); 2];
    xs[0] = [a[0][0], a[1][1], a[2][2]].iter().cloned().collect();
    xs[1] = [a[2][0], a[1][1], a[0][2]].iter().cloned().collect();

    for _ in 0..3 {
        rs.push(HashSet::<i32>::new());
        cs.push(HashSet::<i32>::new());
    }

    for i in 0..3 {
        for j in 0..3 {
            rs[i].insert(a[i][j]);
            cs[j].insert(a[i][j]);
        }
    }

    let mut scores = vec![vec![0; 2]; 3];

    for b_i in &b {
        for t in 0..3 {
            if rs[t].contains(b_i) {
                scores[t][0] += 1;
            }
            if cs[t].contains(b_i) {
                scores[t][1] += 1;
            }
        }
    }

    let mut scores_x = vec![0; 2];
    for b_i in &b {
        if xs[0].contains(b_i) {
            scores_x[0] += 1;
        }
        if xs[1].contains(b_i) {
            scores_x[1] += 1;
        }
    }


    for _scores in scores {
        for _score in _scores {
            if _score == 3 {
                println!("Yes");
                return;
            }
        }
    }

    for _score in scores_x {
        if _score == 3 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
