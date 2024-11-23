use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut s = s.as_str();

    while !s.is_empty() {
        if s.ends_with("dream") {
            s = &s[..s.len() - 5];
        } else if s.ends_with("dreamer") {
            s = &s[..s.len() - 7];
        } else if s.ends_with("erase") {
            s = &s[..s.len() - 5];
        } else if s.ends_with("eraser") {
            s = &s[..s.len() - 6];
        } else {
            println!("NO");
            return;
        }
    }
    println!("YES");
}
