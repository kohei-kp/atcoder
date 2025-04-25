use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let filtered = s.iter().filter(|&&c| c.is_uppercase()).collect::<String>();
    println!("{}", filtered);
}
