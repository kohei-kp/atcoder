use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i32,
        s: String,
    }

    let mut chars = s.chars().rev().collect::<Vec<char>>();
    let mut count = 0;

    for i in 0..n {
        if chars[i] == '@' {
            chars[i] = '.';
            count += 1;
        }

        if count == d {
            break;
        }
    }
    chars.reverse();

    println!("{}", chars.iter().collect::<String>());
}
