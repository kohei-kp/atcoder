use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        a: [[i32;w];h],
        b: [i32; n]
    }

    let mut ans = 0;
    for i in 0..h {
        let mut cnt = 0;
        for j in 0..w {
            if b.contains(&a[i][j]) {
                cnt += 1;
            }
        }
        ans = ans.max(cnt);
    }

    println!("{}", ans);
}
