use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut prev = &s[0];
    for i in 1..n - 1 {
        if prev == "sweet" && s[i] == "sweet" {
            println!("No");
            return;
        }
        prev = &s[i];
    }
    println!("Yes");
}
