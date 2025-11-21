use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut ans = false;
    for i in 0..s.len() {
        if (i + 1) % 2 != 0 {
            continue;
        }
        if s[i] == '0' {
            ans = true;
        } else {
            ans = false;
            break;
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
}
