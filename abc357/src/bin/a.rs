use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: i32,
        h: [i32; n],
    }

    let mut count = 0;
    for i in 0..n {
        m -= h[i];
        if m >= 0 {
            count += 1;
        } else {
            break;
        }
    }
    println!("{}", count);
}
