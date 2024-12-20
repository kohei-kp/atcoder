use proconio::input;

fn main() {
    input! {
        n: usize,
        k: [i32; n],
    }

    let mut ans = 1e9 as i32;

    for s in 0..(1 << n) {
        let mut a = 0;
        let mut b = 0;
        for i in 0..n {
            if s >> i & 1 == 1 {
                b += k[i];
            } else {
                a += k[i];
            }
        }
        let now = a.max(b);
        ans = ans.min(now);
    }

    println!("{}", ans);
}
