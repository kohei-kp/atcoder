use proconio::{input, marker::Chars};
use std::cmp::min;

fn main() {
    input! {
        m: usize,
        s1: Chars,
        s2: Chars,
        s3: Chars,
    }

    let mut ans = 10e9 as i64;
    for t0 in 0..300 {
        for t1 in 0..300 {
            for t2 in 0..300 {
                if t0 == t1 {
                    continue;
                }
                if t0 == t2 {
                    continue;
                }
                if t1 == t2 {
                    continue;
                }

                if s1[t0 % m] != s2[t1 % m] {
                    continue;
                }
                if s1[t0 % m] != s3[t2 % m] {
                    continue;
                }

                let t_max = [t0 as i64, t1 as i64, t2 as i64].into_iter().max().unwrap();
                ans = min(ans, t_max);
            }
        }
    }
    if ans == 10e9 as i64 {
        ans = -1;
    }

    println!("{}", ans);
}
