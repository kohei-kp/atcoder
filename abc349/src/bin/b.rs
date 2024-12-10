use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut map = std::collections::HashMap::new();

    for c in s {
        *map.entry(c).or_insert(0) += 1;
    }

    let mut map2 = std::collections::HashMap::new();
    for (_, v) in map {
        *map2.entry(v).or_insert(0) += 1;
    }

    let values = map2.values().collect::<Vec<_>>();
    if values.iter().all(|&v| v == &2) || values.is_empty() {
        println!("Yes");
    } else {
        println!("No");
    }
}
