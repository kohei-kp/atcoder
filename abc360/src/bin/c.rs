use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        w: [usize; n],
    }

    let mut ws: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    for i in 0..n {
        ws.entry(a[i]).or_insert(vec![]).push(w[i]);
    }

    let empty_count = n - ws.len();

    let mut ss = vec![];
    for (_, ww) in ws.iter_mut() {
        ww.sort();
        for i in 0..ww.len() - 1 {
            ss.push(ww[i]);
        }
    }
    ss.sort();

    println!("{:?}", ss[0..empty_count].iter().sum::<usize>());
}
