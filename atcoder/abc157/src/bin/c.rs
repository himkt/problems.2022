use proconio::input;


fn solve(n: usize, sc: Vec<(usize, i32)>) -> i32 {
    let mut nums = Vec::<i32>::new();
    nums.resize(n, -1);

    for (si, ci) in sc.iter() {
        // specify 0 for the first digit
        if *si == 1 && *ci == 0 && n != 1 {
            return -1;
        }

        if nums[*si-1] == -1 {
            nums[*si-1] = *ci;
        }
        else {
            if nums[*si-1] == *ci {
                continue;
            }
            return -1;
        }
    }

    for i in 0..n {
        if nums[i] == -1 {
            if i == 0 && n > 1 {
                nums[i] = 1;
            }
            else {
                nums[i] = 0;
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        let base = (10 as i32).pow((n - i - 1) as u32);
        ans += nums[i] * base;
    }

    ans
}


fn main() {
    input! {
        n: usize,
        m: usize,
        sc: [(usize, i32); m],
    }

    println!("{}", solve(n, sc));
}
