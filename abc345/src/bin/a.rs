use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let first = s[0];
    let last = s[s.len() - 1];
    println!(
        "{}",
        if first == '<' && last == '>' && s[1..s.len() - 1].iter().all(|&c| c == '=') {
            "Yes"
        } else {
            "No"
        }
    );
}
