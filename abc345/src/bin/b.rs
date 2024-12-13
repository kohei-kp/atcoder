use proconio::input;

fn main() {
    input! {
        x: i64,
    }

    if x < 0 {
        println!("{}", x / 10);
    } else {
        println!("{}", (x + 9) / 10);
    }
}
