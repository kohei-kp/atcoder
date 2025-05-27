use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        m: usize,
        b: [i64; m],
        l: usize,
        c: [i64; l],
        q: usize,
        x: [i64; q],
    }

    let mut tree_set: BTreeSet<i64> = std::collections::BTreeSet::new();
    for i in 0..n {
        for j in 0..m {
            for k in 0..l {
                let sum = a[i] + b[j] + c[k];
                tree_set.insert(sum);
            }
        }
    }

    for i in 0..q {
        let x_value = x[i];
        if tree_set.contains(&x_value) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
