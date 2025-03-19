use proconio::input;

fn main() {
    input! {
        mut n: usize,
        a: [i64; n],
    }

    let mut set = std::collections::BTreeSet::new();
    let mut map = std::collections::BTreeMap::new();
    let mut count = 0;
    for i in 0..n {
        if !set.contains(&a[i]) {
            count += 1;
            set.insert(a[i]);
        }
        map.insert(i, count);
    }

    let mut set_rev = std::collections::BTreeSet::new();
    let mut map_rev = std::collections::BTreeMap::new();
    count = 0;
    for i in (0..n).rev() {
        if !set_rev.contains(&a[i]) {
            count += 1;
            set_rev.insert(a[i]);
        }
        map_rev.insert(n - i, count);
    }

    let mut ans = 0;
    n -= 1;
    map.iter().for_each(|(k, v)| {
        let rev_k = n - k;
        if rev_k == 0 {
            return;
        }
        let _ans = v + map_rev.get(&rev_k).unwrap();
        ans = ans.max(_ans);
    });

    println!("{}", ans);
}
