use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut chars = vec![];
    let mut c = 0;
    for i in 1..s.len() {
        if s[i].is_uppercase() && c > 0 {
            // s[i - 1]
            chars.push(s[i - 1]);
        }
        c += 1;
    }

    let mut all_in_t = true;
    for c in chars {
        if !t.contains(&c) {
            all_in_t = false;
            break;
        }
    }
    if all_in_t {
        println!("Yes");
    } else {
        println!("No");
    }
}
