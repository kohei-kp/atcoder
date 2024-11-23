use proconio::input;

fn main() {
    input! {
        a: i32, // 500 yen
        b: i32, // 100 yen
        c: i32, // 50 yen
        x: i32,
    }

    let mut count = 0;

    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if i * 500 + j * 100 + k * 50 == x {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
