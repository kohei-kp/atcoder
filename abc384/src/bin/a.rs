use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        c1: String,
        c2: String,
        s: Chars
    }

    let mut string = String::new();

    for i in 0..n {
        if s[i] != c1.chars().nth(0).unwrap() {
            string.push_str(&c2);
        } else {
            string.push(s[i]);
        }
    }

    println!("{}", string);
}
