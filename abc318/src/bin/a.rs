use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: usize,
    }

    let mut ans = 0;
    let mut now = m;
    while now <= n {
        ans += 1;
        now += p;
    }

    println!("{}", ans);
}
