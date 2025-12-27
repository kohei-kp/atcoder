use std::{cmp, i64};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
        c: [i64; n],
    }

    let mut sum_a = vec![0i64; n + 1];
    let mut sum_b = vec![0i64; n + 1];
    let mut sum_c = vec![0i64; n + 1];

    for i in 0..n {
        sum_a[i + 1] = sum_a[i] + a[i];
        sum_b[i + 1] = sum_b[i] + b[i];
        sum_c[i + 1] = sum_c[i] + c[i];
    }

    let mut g = vec![0i64; n + 1];
    let mut h = vec![0i64; n + 1];
    for i in 0..n + 1 {
        g[i] = sum_a[i] - sum_b[i];
        h[i] = sum_b[i] - sum_c[i];
    }

    let mut ans = 0;
    let mut mx = -i64::MAX;

    for i in 1..n {
        ans = cmp::max(ans, mx + h[i] + sum_c[n]);
        mx = cmp::max(mx, g[i]);
    }

    println!("{}", ans);
}
