use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        y: i64,
        a: [i64; n],
        b: [i64; n],
    }

    let mut ab = a.into_iter().zip(b.into_iter()).collect::<Vec<_>>();
    if x < y {
        ab.sort_by(|a, b| a.0.cmp(&b.0));
    } else {
        ab.sort_by(|a, b| a.1.cmp(&b.1));
    }

    ab.reverse();

    let mut sum_a = 0;
    let mut sum_b = 0;
    let mut ans = 0;

    for (i, v) in ab.iter().enumerate() {
        sum_a += v.0;
        sum_b += v.1;
        if sum_a > x || sum_b > y {
            ans = i + 1;
            break;
        }
    }
    if ans == 0 {
        ans = n;
    }

    println!("{}", ans);
}
