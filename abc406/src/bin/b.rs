use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    }

    let mut ans: i64 = 1;
    for i in 0..n {
        if ans.checked_mul(a[i]).is_none() {
            ans = 1;
        } else {
            ans *= a[i];
        }
        if ans.to_string().len() > k {
            ans = 1;
        }
    }
    println!("{}", ans);
}
