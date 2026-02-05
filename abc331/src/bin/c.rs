use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut a_clone = a.clone();
    a_clone.sort_by(|a, b| b.cmp(&a));
    let mut map = HashMap::new();

    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + a_clone[i];
        map.entry(a_clone[i]).or_insert(i);
    }

    for i in 0..n {
        let count = map.get(&a[i]).unwrap();
        print!("{} ", sum[*count]);
    }
    println!();
}
