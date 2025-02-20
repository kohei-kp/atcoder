use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut sides = HashSet::new();

    let mut ans = 0;

    for _ in 0..m {
        input! {
            a: i64,
            b: i64,
        }

        if a == b || sides.contains(&format!("{}-{}", a.min(b), a.max(b))) {
            ans += 1;
        } else {
            sides.insert(format!("{}-{}", a.min(b), a.max(b)));
        }
    }

    println!("{}", ans);
}
