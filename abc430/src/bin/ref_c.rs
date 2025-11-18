use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars
    }

    // aとbについて、それぞれ累積和をとる
    let mut sum_a = vec![0; n + 1];
    let mut sum_b = vec![0; n + 1];
    for i in 0..n {
        if s[i] == 'a' {
            sum_a[i + 1] = 1;
        } else {
            sum_b[i + 1] = 1;
        }
    }
    for i in 0..n {
        sum_a[i + 1] += sum_a[i];
        sum_b[i + 1] += sum_b[i];
    }

    let mut ans = 0;
    for l in 0..n {
        let ra;
        let rb;
        {
            let mut wa = l;
            let mut ac = n + 1;
            while ac - wa > 1 {
                let wj = (wa + ac) / 2;
                if sum_a[wj] - sum_a[l] >= a {
                    ac = wj;
                } else {
                    wa = wj;
                }
            }
            ra = ac;
        }
        {
            let mut wa = n + 1;
            let mut ac = l;
            while wa - ac > 1 {
                let wj = (wa + ac) / 2;
                if sum_b[wj] - sum_b[l] < b {
                    ac = wj;
                } else {
                    wa = wj;
                }
            }
            rb = wa;
        }
        ans += std::cmp::max(0, rb as isize - ra as isize);
    }

    // println!("{:?}", sum_a);
    // println!("{:?}", sum_b);
    println!("{}", ans);
}
