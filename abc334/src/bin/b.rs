use proconio::input;

fn main() {
    input! {
        mut a: i64,
        mut m: i64,
        mut l: i64,
        mut r: i64,
    }

    // Aの分ずらす
    l -= a;
    r -= a;

    // LとRを正に補正する
    if l < 0 {
        let x = -l / m + 1;
        l += x * m;
        r += x * m;
    }

    let ans = f(r, m) - f(l - 1, m);

    println!("{}", ans);
}

fn f(r: i64, m: i64) -> i64 {
    r / m
}
