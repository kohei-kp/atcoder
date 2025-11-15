use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        a: String,
        b: String,
        c: String,
    }

    let ans = vec![a, b, c]
        .into_iter()
        .permutations(3)
        .map(|p| p.join(""))
        .max()
        .unwrap();

    println!("{}", ans);
}
