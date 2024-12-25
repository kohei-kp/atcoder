use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }

    let mut ans = 0;
    for w in 1..=n {
        for si in 0..w {
            let mut a = vec![];
            for i in (si..n).step_by(w) {
                a.push(h[i]);
            }

            let mut val = -1;
            let mut len = 0;
            for &x in a.iter() {
                if val == x {
                    len += 1;
                } else {
                    val = x;
                    len = 1;
                }
                ans = ans.max(len);
            }
        }
    }

    println!("{}", ans);
}
