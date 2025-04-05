use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let mut ans = 0;

    // 2*60は10^18より大きいので、60回までで十分
    for a in 1..60 {
        let m = isqrt(n >> a);
        // 奇数の数 m + 1 / 2
        ans += (m + 1) / 2;
    }

    println!("{}", ans);
}

// 整数の平方根を求める
fn isqrt(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut x = n;
    let mut y = (x + 1) / 2;

    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }

    x
}
