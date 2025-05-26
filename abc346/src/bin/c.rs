use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let mut sum = k * (k + 1) / 2;
    let mut set = std::collections::HashSet::new();

    for i in 0..n {
        if 1 <= a[i] && a[i] <= k {
            if set.contains(&a[i]) {
                continue;
            }
            sum -= a[i];
            set.insert(a[i]);
        }
    }

    println!("{}", sum);
}
