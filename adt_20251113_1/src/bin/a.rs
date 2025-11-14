use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut flg = true;
    for i in 0..s.len() {
        if i == 0 && s[i] != '<' {
            flg = false;
        } else if i > 0 && i < s.len() - 1 && s[i] != '=' {
            flg = false;
        } else if i == s.len() - 1 && s[i] != '>' {
            flg = false;
        }
    }

    println!("{}", if flg { "Yes" } else { "No" });
}
