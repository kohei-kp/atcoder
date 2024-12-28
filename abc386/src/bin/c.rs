use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut k: i32,
        s: String,
        t: String
    }

    let _s = s.chars().collect::<Vec<_>>();
    let _t = t.chars().collect::<Vec<_>>();
    // s.len t.lenの2次元配列

    let mut board = vec![vec![0; s.len()]; t.len()];
    println!("{:?}", board);
}
