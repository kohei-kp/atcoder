use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }

    let mut a = vec![0; k.max(n + 1)];
    // 累積和
    let mut s = vec![0; a.len() + 1];

    for i in 0..k {
        a[i] = 1;
    }
    for i in 0..k {
        s[i + 1] = s[i] + a[i];
    }

    let _mod = 1e9 as i64;
    for i in k..=n {
        a[i] = (s[i] - s[i - k] + _mod) % _mod;
        s[i + 1] = (s[i] + a[i]) % _mod;
    }

    println!("{}", a[n]);
}
