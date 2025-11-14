use proconio::input;

fn main() {
    input! {
        n: usize,
        qr: [(i64, i64); n],
        q: usize,
        td: [(i64, i64); q],
    }

    for i in 0..q {
        let t = td[i].0 as usize - 1;
        let mut d = td[i].1;

        d -= qr[t].1;
        let mut ans = (d + qr[t].0 - 1) / qr[t].0 * qr[t].0;
        ans += qr[t].1;
        println!("{}", ans);
    }
}
