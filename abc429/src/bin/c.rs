use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut number_counts = HashMap::new();
    for &val in &a {
        *number_counts.entry(val).or_insert(0) += 1;
    }

    let mut result = 0;
    for (_, &count) in &number_counts {
        if count >= 2 {
            let combinations = count * (count - 1) / 2;
            let other = n - count;
            result += combinations * other;
        }
    }
    println!("{}", result);
}
