use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ch1 = s.chars().nth(0).unwrap();
    let other = s.chars().skip(1).collect::<String>();

    let all_lower = other.chars().all(|c| c.is_lowercase());

    if ch1.is_uppercase() && all_lower {
        println!("Yes");
    } else {
        println!("No");
    }
}
