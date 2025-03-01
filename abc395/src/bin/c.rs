use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut map = std::collections::HashMap::new();

    for i in 0..n {
        let entry = map.entry(a[i]).or_insert(vec![]);
        entry.push(i);
    }

    let mut distance = n;
    for (_, v) in map {
        if v.len() == 1 {
            continue;
        }

        for i in 0..v.len() - 1 {
            distance = std::cmp::min(distance, v[i + 1] - v[i]);
        }
    }

    println!(
        "{}",
        if distance == n {
            -1
        } else {
            distance as i64 + 1
        }
    );
}
