use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    s.iter().for_each(|c| {
        if c == &'2' {
            print!("{}", c);
        }
    });
}
