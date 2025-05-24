use proconio::input;

fn main() {
    input! {
        x: f64,
        y: f64,
    }

    let max = 6 * 6; // サイコロの目の組み合わせは36通り
    let mut count = 0;

    // 1から6まで（サイコロの目）の確率を計算
    for i in 1..=6 {
        for j in 1..=6 {
            // i + jが x以上のパターン or iとjの差がy以上のパターンをカウント
            if (i + j >= x as i64) || (i as i64 - j as i64).abs() >= y as i64 {
                count += 1;
            }
        }
    }

    println!("{}", count as f64 / max as f64);
}
