use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut chars = HashMap::<char, usize>::new();

    for c in s.chars() {
        let count = chars.entry(c).or_insert(0);
        *count += 1;
    }

    let max = chars.values().max().unwrap();
    let mut max_chars = chars
        .iter()
        .filter(|(_, v)| *v == max)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();

    max_chars.sort();

    println!("{}", max_chars[0]);
}
