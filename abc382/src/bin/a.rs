use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i32,
        s: String,
    }

    let mut chars = s.chars().collect::<Vec<char>>();
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

    println!("{}", chars.iter().filter(|&c| c == &'.').count());
}
