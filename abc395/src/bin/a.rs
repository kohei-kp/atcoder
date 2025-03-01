use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut prev = a[0];
    for i in 1..n {
        if prev >= a[i] {
            println!("No");
            return;
        }
        prev = a[i];
    }

    println!("Yes");
}
