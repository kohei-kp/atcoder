use proconio::input;

fn main() {
    input! {
        l: i64,
        r: i64,
    }

    let ans = f(r) - f(l - 1);
    println!("{}", ans);
}

fn pw(x: i64, p: i64) -> i64 {
    let mut res = 1;
    for _ in 0..p {
        res *= x;
    }
    res
}

fn f(mut r: i64) -> i64 {
    r += 1;
    let mut digits = vec![];
    for c in r.to_string().chars() {
        digits.push(c.to_digit(10).unwrap() as i64);
    }

    let n = digits.len() as i64;

    let mut res = 0;
    for k in 1..n {
        for h in 1..=9 {
            res += pw(h, k - 1);
        }
    }

    for h in 1..digits[0] {
        res += pw(h, n - 1);
    }

    let h = digits[0];
    for i in 1..n {
        if digits[i as usize] >= h {
            res += pw(h, n - i);
            break;
        }
        res += pw(h, n - i - 1) * digits[i as usize];
    }

    res
}
