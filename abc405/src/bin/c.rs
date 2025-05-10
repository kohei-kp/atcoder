use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }

    let mut sum = vec![0; n + 1];
    for i in (0..n).rev() {
        sum[i] = sum[i + 1] + a[i];
    }

    let mut ans = 0;
    for i in 0..n {
        ans += (sum[i] - a[i]) * a[i];
    }

    println!("{}", ans);
}
