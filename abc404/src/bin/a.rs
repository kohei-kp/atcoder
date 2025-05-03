use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let alphabets = "abcdeefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();

    for i in alphabets {
        if !s.contains(&i) {
            println!("{}", i);
            return;
        }
    }
}
