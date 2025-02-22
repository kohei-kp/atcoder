use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut ans = String::new();
    let mut w_count = 0;

    for i in 0..s.len() {
        if s[i] == 'W' {
            w_count += 1;
        } else {
            if w_count > 0 && s[i] == 'A' {
                ans.push_str(&format!("A{}", "C".repeat(w_count)));
            } else {
                ans.push_str(&format!(
                    "{}{}",
                    "W".repeat(w_count),
                    s[i].to_string().as_str()
                ));
            }
            w_count = 0;
        }
    }
    if w_count > 0 {
        ans.push_str(&format!("{}", "W".repeat(w_count)));
    }
    println!("{}", ans);
}
