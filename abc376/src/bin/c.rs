use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
        mut b: [i32; n - 1],
    }

    a.sort();
    b.sort();

    let judge = |x: i32| {
        let mut nb = b.clone();
        nb.push(x);
        nb.sort();

        for i in 0..n {
            if a[i] > nb[i] {
                return false;
            }
        }
        true
    };

    let inf = 1001001001;
    let mut ac = inf;
    let mut wa = 0;

    while ac - wa > 1 {
        let wj = (ac + wa) / 2;
        if judge(wj) {
            ac = wj;
        } else {
            wa = wj;
        }
    }

    if ac == inf {
        println!("-1");
    } else {
        println!("{}", ac);
    }
}
