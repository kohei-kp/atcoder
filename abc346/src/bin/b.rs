use proconio::input;

fn main() {
    input! {
        w: i32,
        b: i32,
    }

    if w == 0 && b >= 2 {
        println!("No");
        return;
    }
    if b == 0 && w >= 3 {
        println!("No");
        return;
    }

    let s = "wbwbwwbwbwbw"
        .repeat(50)
        .as_str()
        .chars()
        .collect::<Vec<char>>();

    for i in 0..s.len() {
        for j in i + 1..s.len() + 1 {
            let _s = &s[i..j];
            let mut w_count = 0;
            let mut b_count = 0;
            for _s in _s {
                if _s == &'w' {
                    w_count += 1;
                } else {
                    b_count += 1;
                }
            }
            if w_count == w && b_count == b {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
