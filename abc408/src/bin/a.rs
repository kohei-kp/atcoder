use proconio::input;

fn main() {
    input! {
        n: usize,
        s: f64,
        t: [f64; n],
    }

    let interval = s + 0.5;

    let mut bool = true;
    let mut prev = 0.0;
    for i in 0..n {
        if t[i] - prev >= interval {
            bool = false;
            break;
        }
        prev = t[i];
    }

    println!("{}", if bool { "Yes" } else { "No" });
}
