use proconio::input;

fn main() {
    input! {
        s: [String; 12]
    }

    let mut count = 0;
    for i in 0..12 {
        if s[i].len() == i + 1 {
            count += 1;
        }
    }
    println!("{}", count);
}
