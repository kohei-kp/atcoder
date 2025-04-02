use proconio::input;

fn main() {
    input! {
        n: usize,
        mut h: [i64; n],
    }

    let mut t: i64 = 0;

    for i in 0..n {
        // 1 1 3の周期を利用する
        // 1サイクルで5減る。
        let x = h[i] / 5;
        t += x * 3;
        h[i] -= x * 5;
        while h[i] > 0 {
            t += 1;
            if t % 3 == 0 {
                h[i] -= 3;
            } else {
                h[i] -= 1;
            }
        }
    }

    println!("{}", t);
}
