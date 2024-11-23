use proconio::input;

fn main() {
    input! {
        n: i32,
        y: i32,
    }

    let i = 10000;
    let j = 5000;
    let k = 1000;

    let mut res = None;

    for a in 0..=n {
        for b in 0..=n - a {
            let c = n - a - b;
            if i * a + j * b + k * c == y {
                res = Some((a, b, c));
                break;
            }
        }
    }

    if let Some((a, b, c)) = res {
        println!("{} {} {}", a, b, c);
    } else {
        println!("-1 -1 -1");
    }
}
