use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut black: [i64; n],
        mut white: [i64; m],
    }

    // 降順に
    black.sort_by(|a, b| b.cmp(a));
    white.sort_by(|a, b| b.cmp(a));

    let inf = -1_000_000_001;
    if n > m {
        for _ in 0..(n - m) {
            white.push(inf);
        }
    } else {
        for _ in 0..(m - n) {
            black.push(inf);
        }
    }

    let mut ans = 0;
    let mut b_count = 0;
    let mut w_count = 0;
    let mut _ans = vec![];
    for i in 0..black.len() {
        if black[i] != inf {
            ans += black[i];
            b_count += 1;

            if b_count >= w_count {
                _ans.push(ans);
            }
        }

        if white[i] != inf {
            if ans + white[i] <= ans {
                continue;
            }
            ans += white[i];
            w_count += 1;
            if b_count >= w_count {
                _ans.push(ans);
            }
        }
    }

    let max = _ans.iter().max().unwrap();

    if max < &0 {
        println!("0");
    } else {
        println!("{}", max);
    }
}
