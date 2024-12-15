use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    for i in 0..s.len() {
        let mut is_diff = true;
        for j in 0..s.len() {
            if i != j && s[i] == s[j] {
                is_diff = false;
            }
        }
        if is_diff {
            println!("{}", i + 1);
            return;
        }
    }
}
