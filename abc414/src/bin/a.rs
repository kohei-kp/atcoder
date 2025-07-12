use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        xy: [(usize, usize); n],
    }

    let mut ans = 0;

    for i in 0..n {
        let (x, y) = xy[i];

        if x < l || x == l {
            if y >= r {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
