use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; m]
    }

    let mut prev = 0;
    let mut map = BTreeMap::new();
    for i in 0..m {
        let day = a[i];

        for j in prev..day {
            map.insert(j + 1, day);
        }

        prev = day;
    }

    for i in 0..n {
        let key: i64 = i as i64 + 1;
        println!("{}", map.get(&key).unwrap() - key)
    }
}
