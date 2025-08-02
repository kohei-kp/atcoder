use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars
    }

    let mut ans_s = String::new();
    for i in 0..n {
        if i < a {
            continue;
        } else if i > (n - 1) - b {
            continue;
        }
        ans_s.push(s[i]);
    }
    println!("{}", ans_s);
}
