use proconio::input;


fn main() {
    input! {
        a: i64,
        b: i64,
        c: usize,
    };

    if a == b {
        println!("=");
        return;
    }

    if a == 0 && b == 0 {
        println!("=");
        return;
    }

    if a == 0 && b < 0 {
        if c % 2 == 0 {
            println!("<");
            return;
        }
        else {
            println!(">");
            return;
        }
    }

    if a == 0 && b > 0 {
        println!("<");
        return;
    }

    if a < 0 && b == 0 {
        if c % 2 == 0 {
            println!(">");
            return;
        }
        else {
            println!("<");
            return;
        }
    }

    if a > 0 && b == 0 {
        println!(">");
        return;
    }

    // a != 0 && b != 0

    // 肩が偶数 => pow の結果は正
    if c % 2 == 0 {
        if a.abs() == b.abs() {
            println!("=");
            return;
        }
        else if a.abs() > b.abs() {
            println!(">");
            return;
        }
        else if a.abs() < b.abs() {
            println!("<");
            return;
        }
    }
    // 肩が奇数 => pow の結果が負になりうる
    else {
        // 符号が同じ && 肩が奇数
        if a * b > 0 {
            if a > 0 {
                if a > b {
                    println!(">");
                }
                else {
                    println!("<");
                }
            }
            // a < 0 => b > 0
            else {
                if a > b {
                    println!("<");
                }
                else {
                    println!(">");
                }
            }
        }
        // 符号が異なる
        else {
            if a > 0 {
                println!(">");
            }
            else if b > 0 {
                println!("<");
            }
        }
    }
}
