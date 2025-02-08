use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        p: [i64; n],
        q: [i64; n],
    }

    let mut z_order: BTreeMap<i64, (i64, i64)> = BTreeMap::new();
    let mut p_order: BTreeMap<i64, i64> = BTreeMap::new();
    for i in 0..n {
        z_order.insert(q[i], ((i + 1) as i64, p[i]));
        p_order.insert((i + 1) as i64, q[i]);
    }

    for (i, (k, v)) in z_order.iter().enumerate() {
        let p_v = p_order.get(&v.1).unwrap();
        print!("{} ", p_v);
    }
}
