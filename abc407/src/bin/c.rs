use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars
    }

    let mut ans = 0;
    let mut x = 0;
    while let Some(c) = s.pop() {
        loop {
            let d = c.to_digit(10).unwrap() as i32;
            if (d - x) % 10 == 0 {
                break;
            }
            x += 1;
            ans += 1;
        }
        ans += 1;
    }

    println!("{}", ans);
}
