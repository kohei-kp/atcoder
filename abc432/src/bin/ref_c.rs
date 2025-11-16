use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        y: i64,
        mut a: [i64; n]
    }

    // 一番もらえる飴の数が少ない人は全部大きな飴で良い。
    // ソートしてa[0] * yの重さを出す
    a.sort();
    let w = a[0] * y;
    let mut ans = 0;
    for i in 0..n {
        let sw = a[i] * y;
        let dif = sw - w;
        if dif % (y - x) != 0 {
            println!("-1");
            return;
        }

        let num = dif / (y - x);
        if num > a[i] {
            println!("-1");
            return;
        }

        ans += a[i] - num;
    }

    println!("{}", ans);
}
