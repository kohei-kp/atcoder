use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [i64; n],
    }

    a.sort();

    let mut ans = 1e9 as i64;
    for l in 0..k + 1 {
        let r = l + (n - k) - 1;
        let now = a[r] - a[l];
        ans = ans.min(now);
    }

    println!("{}", ans);
}
