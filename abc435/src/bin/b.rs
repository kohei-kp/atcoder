use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }

    let mut ans = 0;

    for l in 0..n {
        for r in l..n {
            let mut sum = 0;
            for i in l..=r {
                sum += a[i];
            }

            let mut ok = true;
            for i in l..=r {
                if sum % a[i] == 0 {
                    ok = false;
                    break;
                }
            }

            if ok {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
