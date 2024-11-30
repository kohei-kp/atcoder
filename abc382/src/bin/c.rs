// 解説を読んで実装
use proconio::input;

const K: i32 = 200010;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i32; n],
        b: [i32; m],
    }

    let mut r = K;
    let mut id = vec![-1; K as usize];

    for i in 0..n {
        while r > a[i] {
            r -= 1;
            id[r as usize] = (i + 1) as i32;
        }
    }

    for i in 0..m {
        println!("{}", id[b[i] as usize]);
    }
}
