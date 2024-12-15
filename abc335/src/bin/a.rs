use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut _s = s.clone();
    _s[s.len() - 1] = '4';
    println!("{}", _s.iter().collect::<String>());
}
