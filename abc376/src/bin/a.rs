use proconio::input;

fn main() {
    input! {
        n: usize,
        c: i32,
        t: [i32; n],
    }

    let mut ans = 1;
    let mut prev = t[0];

    for i in 1..n {
        if t[i] - prev >= c {
            ans += 1;
            prev = t[i];
        }
    }

    println!("{}", ans);
}
