use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            h: i64,
        }

        let mut points = vec![];
        for _ in 0..n {
            input! {
                case: (i64, i64, i64)
            }
            points.push(case);
        }

        let mut t = 0;
        let mut l = h;
        let mut u = h;

        let mut flg = true;
        for p in points {
            // 最大移動できる距離(T0->T1)
            let cost = p.0 - t;
            t = p.0;
            l -= cost;
            u += cost;

            // 飛行可能範囲内ならそのまま。
            // そうでないなら、目標範囲に訂正する
            l = max(l, p.1);
            u = min(u, p.2);
            // 逆転するなら不可
            if l > u {
                flg = false;
                break;
            }
        }

        println!("{}", if flg { "Yes" } else { "No" });
    }
}
