use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    for i in 0..n {
        if (i + 1) % 2 == 1 {
            ans += a[i];
        }
    }

    println!("{}", ans);
}
