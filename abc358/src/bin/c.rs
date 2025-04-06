use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let mut ans = n;
    // 2^n
    for x in 0..1 << n {
        let mut ok = true;

        for i in 0..m {
            let mut can = false;

            for j in 0..n {
                if (x >> j) & 1 == 1 && s[j][i] == 'o' {
                    can = true;
                }
            }
            if !can {
                ok = false;
            }
        }
        if ok {
            ans = ans.min(((x as u32).count_ones()) as usize);
        }
    }

    println!("{}", ans);
}
