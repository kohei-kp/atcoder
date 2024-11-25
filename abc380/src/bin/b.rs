use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let s = s.chars().collect::<Vec<char>>();

    let mut counts = vec![];

    let mut count = 0;

    for i in 0..s.len() {
        if i == 0 && s[i] == '|' {
            continue;
        }
        if s[i] == '-' {
            count += 1;
        } else {
            counts.push(count);
            count = 0;
        }
    }

    println!(
        "{}",
        counts
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
