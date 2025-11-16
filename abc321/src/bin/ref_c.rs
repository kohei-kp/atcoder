use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let mut a: Vec<i64> = vec![];

    // BIT全探索
    for s in 0..(1 << 10) {
        let mut x: i64 = 0;
        for i in (0..10).rev() {
            if s >> i & 1 == 1 {
                x = x * 10 + i;
            }
        }
        if x == 0 {
            continue;
        }
        a.push(x);
    }
    a.sort();

    println!("{}", a[k - 1]);
}
