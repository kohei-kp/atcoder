use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let s = if n % 2 == 0 { "==" } else { "=" };

    let ans = format!(
        "{}{}{}",
        "-".repeat((n - s.len()) / 2),
        s,
        "-".repeat((n - s.len()) / 2)
    );
    println!("{}", ans);
}
