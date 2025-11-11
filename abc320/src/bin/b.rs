use proconio::{input, marker::Chars};
use std::cmp::max;

fn main() {
    input! {
        s: Chars
    }

    let n = s.len();
    let mut ans = 0;

    for i in 0..n {
        for j in i + 1..n + 1 {
            let ss = s[i..j].iter().collect::<String>();

            if ss == ss.chars().rev().collect::<String>() {
                ans = max(ans, ss.len());
            }
        }
    }

    println!("{}", ans);
}
