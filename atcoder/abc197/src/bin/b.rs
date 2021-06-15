use proconio::input;


fn solve (s: Vec<Vec<char>>, h: usize, w: usize, x: usize, y: usize) -> usize {
    let mut contain_self = false;
    let mut ans = 0;
    let mut cnt = 0;

    // fixed x
    for j in 0..w {
        if j == y {
            contain_self = true;
        }

        if s[x][j] == '.' {
            cnt += 1;
            // println!("add ({}, {}), contain_self: {}", x, j, contain_self);
        }
        else {
            if contain_self {
                ans += cnt;
                cnt = 0;
                break;
            }
            cnt = 0;
        }
    }

    ans += cnt;
    contain_self = false;
    cnt = 0;

    // fixed y
    for i in 0..h {
        if i == x {
            contain_self = true;
        }

        if s[i][y] == '.' {
            cnt += 1;
            // println!("add ({}, {}), contain_self: {}", i, y, contain_self);
        }
        else {
            if contain_self {
                ans += cnt;
                cnt = 0;
                break;
            }
            cnt = 0;
        }
    }

    ans += cnt;
    // -1 for self point.
    ans - 1
}


fn main() {
    input! {
        h: usize,
        w: usize,
        x: usize,
        y: usize,
        s: [String; h],
    }

    let s = s
        .iter()
        .map( |l| l.chars().collect()).collect();

    let ans = solve(s, h, w, x-1, y-1);
    println!("{}", ans);
}
