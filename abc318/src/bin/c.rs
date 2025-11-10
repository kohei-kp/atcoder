use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        n: usize,
        d: i64, // D枚セット
        p: i64, // P円
        mut f: [i64; n] // F[i] = 通常料金
    }

    f.sort();

    // 累積和
    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + f[i];
    }

    let mut ans = 1e18 as i64 + 7;
    let mut i: i64 = 0;
    loop {
        let r: i64 = max(0, n as i64 - i * d);
        let now = sum[r as usize] + p * i;
        ans = min(ans, now);
        if r == 0 {
            break;
        }
        i += 1;
    }

    println!("{}", ans);
}
