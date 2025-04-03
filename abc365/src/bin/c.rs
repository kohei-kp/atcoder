use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i64,
        mut a: [i64; n],
    }

    let inf = 1001001001;
    if f(inf, &a, m) {
        println!("infiniite");
    } else {
        let mut ac = 0;
        let mut wa = inf;
        while ac + 1 < wa {
            let wj = (ac + wa) / 2;
            if f(wj, &a, m) {
                ac = wj;
            } else {
                wa = wj;
            }
        }
        println!("{}", ac);
    }
}

fn f(x: i64, a: &[i64], m: i64) -> bool {
    let mut s: i64 = 0;
    for i in 0..a.len() {
        s += x.min(a[i]);
    }
    s <= m
}
