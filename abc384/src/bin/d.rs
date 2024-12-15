use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        mut s: i64,
        mut a: [i64; n],
    }

    let sum = a.iter().sum::<i64>();
    let s = s % sum;

    a.extend_from_within(..);

    let mut prefix_sum = BTreeSet::new();
    prefix_sum.insert(0i64);
    let mut p = 0i64;

    for i in 0..n * 2 {
        p += a[i];
        prefix_sum.insert(p);
    }

    for &p in &prefix_sum {
        if prefix_sum.contains(&(p + s)) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
