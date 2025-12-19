use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let mut map = std::collections::HashMap::new();
    let mut at = 0;

    for i in 0..s.len() {
        if s[i] == '@' {
            at += 1;
        } else {
            *map.entry(s[i]).or_insert(0) += 1;
        }
        if t[i] == '@' {
            at += 1;
        } else {
            *map.entry(t[i]).or_insert(0) -= 1;
        }
    }

    for (k, v) in map {
        if v != 0 {
            if !"atcoder".contains(k) {
                println!("No");
                return;
            } else {
                at -= (v as i32).abs();
            }
        }
    }

    if at >= 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
