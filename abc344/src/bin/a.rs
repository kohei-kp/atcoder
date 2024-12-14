use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let indexes = s.char_indices().filter(|(_, c)| c == &'|');

    println!(
        "{}{}",
        &s[0..indexes.clone().next().unwrap().0],
        &s[indexes.clone().last().unwrap().0 + 1..]
    );
}
