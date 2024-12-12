use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut ans = 0;

    for i in (0..s.len()).rev() {
        println!("i: {}", i);
    }
}
