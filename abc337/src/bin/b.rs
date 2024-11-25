use proconio::input;
use std::collections::HashMap;

fn main() {
    input!(s: String);

    let mut chars = HashMap::<char, usize>::new();
    chars.insert('A', 0);
    chars.insert('B', 0);
    chars.insert('C', 0);

    for s in s.chars() {
        *chars.entry(s).or_insert(0) += 1;
    }

    let a = "A".to_string().repeat(*chars.get(&'A').unwrap());
    let b = "B".to_string().repeat(*chars.get(&'B').unwrap());
    let c = "C".to_string().repeat(*chars.get(&'C').unwrap());

    let result = format!("{}{}{}", a, b, c);

    if result == s {
        println!("Yes");
    } else {
        println!("No");
    }
}
