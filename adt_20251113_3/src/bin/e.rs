use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [String; n]
    }

    let mut ans = 0;
    for i in 1..=n {
        let perm = s.iter().combinations(i);
        for p in perm {
            let mut map = BTreeMap::new();
            for ii in 0..p.len() {
                for c in p[ii].chars() {
                    let count = map.entry(c).or_insert(0);
                    *count += 1;
                }
            }
            let mut count = 0;
            for (kk, v) in map {
                if v == k {
                    count += 1;
                }
            }
            ans = std::cmp::max(ans, count);
        }
    }

    println!("{}", ans);
}
