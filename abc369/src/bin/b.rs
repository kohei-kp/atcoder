use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [(i32, String); n],
    }

    let mut x = 0;
    let mut y = 0;
    let mut count = 0;

    for i in 0..n {
        let (key, lr) = a[i].clone();

        if lr == "L" {
            if x != 0 {
                count += (x - key).abs();
            }
            x = key;
        } else {
            if y != 0 {
                count += (y - key).abs();
            }
            y = key;
        }
    }

    println!("{}", count);
}
