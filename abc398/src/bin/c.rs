use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }

    let mut tree_map = std::collections::BTreeMap::new();
    let mut n2p = std::collections::BTreeMap::new();

    for i in 0..n {
        n2p.insert(a[i], i);

        let count = tree_map.entry(a[i]).or_insert(0);
        *count += 1;
    }

    let mut max: i64 = 0;
    for (k, v) in tree_map.iter_mut() {
        if *v == 1 {
            max = std::cmp::max(max, *k);
        }
    }

    if max == 0 {
        println!("-1");
    } else {
        let p = n2p.get(&max).unwrap();
        println!("{}", p + 1);
    }
}
