use proconio::{input, marker::Chars};
use std::collections::BTreeMap;

fn main() {
    input! {
        s: Chars
    }

    let mut map = BTreeMap::new();
    for i in 0..s.len() {
        let c = s[i];
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }

    let mut ans = 0;
    let mut ans_c = "".to_string();
    for (k, v) in &map {
        if ans < *v {
            ans_c = k.to_string();
            ans = *v;
        }
    }

    println!("{}", ans_c);
}
