use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
    }

    let mut x = vec![0; n];
    for i in 0..n - 1 {
        input! {
            d: usize,
        }
        x[i + 1] = (x[i] + d) % l;
    }
    let mut cnt = vec![0; l];
    for i in 0..n {
        cnt[x[i]] += 1;
    }

    if l % 3 != 0 {
        println!("0");
        return;
    }

    let r = l / 3;
    let mut ans: i64 = 0;
    for a in 0..n {
        let x1 = (x[a] + r) % l;
        let x2 = (x1 + r) % l;
        ans += cnt[x1] as i64 * cnt[x2] as i64;
    }
    println!("{}", ans / 3);
}
