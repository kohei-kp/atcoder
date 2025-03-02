use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut ans: i64 = 0;
    let mut r = 0;

    for l in 0..n {
        while r < n {
            if r > l + 1 && a[r] - a[r - 1] != a[r - 1] - a[r - 2] {
                break;
            }
            r += 1;
        }
        ans += r as i64 - l as i64;
    }

    println!("{}", ans);
}
