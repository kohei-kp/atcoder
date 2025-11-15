use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars
    }

    let ans = x
        .iter()
        .permutations(x.len())
        .filter(|p| p[0] != &'0')
        .map(|p| p.into_iter().collect::<String>().parse::<i64>().unwrap())
        .min()
        .unwrap();

    println!("{}", ans);
}
