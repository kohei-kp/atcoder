use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut upper_count = 0;
    let mut lower_count = 0;
    s.chars().for_each(|c| {
        if c.is_uppercase() {
            upper_count += 1;
        } else {
            lower_count += 1;
        }
    });
    if upper_count > lower_count {
        println!("{}", s.to_uppercase());
    } else {
        println!("{}", s.to_lowercase());
    }
}
