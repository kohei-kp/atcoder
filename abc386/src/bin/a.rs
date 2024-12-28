use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        cards: [i32; 4]
    }

    let mut map = HashMap::new();
    for card in cards {
        let count = map.entry(card).or_insert(0);
        *count += 1;
    }

    let mut values = map.values().collect::<Vec<_>>();
    values.sort();

    let is_ok = match values.len() {
        2 => (values[0] == &1 && values[1] == &3) || (values[0] == &2 && values[1] == &2),
        _ => false,
    };

    println!("{}", if is_ok { "Yes" } else { "No" });
}
