use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars
    }

    let ans = s.into_iter().skip(a).take(n - a - b).collect::<String>();
    println!("{}", ans);
}
