use proconio::input;

fn main() {
    input! {
        n: i32,
        s: i32,
        m: i32,
        l: i32
    }

    let mut ans = 100000000;
    for i in 0..101 {
        for j in 0..101 {
            for k in 0..101 {
                if i * 6 + j * 8 + k * 12 >= n {
                    ans = ans.min(i * s + j * m + k * l);
                }
            }
        }
    }
    println!("{}", ans);
}
