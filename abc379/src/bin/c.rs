use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut stones: Vec<(i64, i64)> = vec![];

    // X, Aをペアで格納していく
    for _ in 0..m {
        input! {
            a: i64,
        }
        stones.push((a, 0));
    }
    for i in 0..m {
        input! {
            b: i64,
        }
        stones[i].1 = b;
    }

    // Xが昇順であるとは限らないので、ソートする
    stones.sort_by(|a, b| a.0.cmp(&b.0));
    // 座標N+1にも石があると考える
    stones.push(((n + 1) as i64, 1));

    let mut ans = 0i64;
    let mut px = 0; // ブロックごとの開始座標
    let mut num = 1i64;
    for (x, a) in stones {
        let len = x - px;
        let carry = num - len;
        // ここが操作回数
        ans += (len - 1) * len / 2; // 等差数列の和
        ans += len * carry; // 余分な石の数

        // 石が足りないなら終了
        if carry < 0 {
            println!("-1");
            return;
        }

        px = x;
        num = carry + a;
    }

    if num != 1 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
