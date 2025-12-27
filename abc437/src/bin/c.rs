use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            mut wp: [(i64, i64); n]
        }

        wp.sort_by_key(|&(a, b)| a + b);
        let mut remain = 0;
        for (_i, v) in wp.iter().enumerate() {
            remain += v.1;
        }

        let mut ans = n;
        for i in 0..n {
            let (w, p) = wp[i];
            remain -= w + p;
            if remain < 0 {
                ans = i;
                break;
            }
        }
        println!("{}", ans);
    }
}
