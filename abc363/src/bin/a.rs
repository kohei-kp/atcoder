use proconio::input;

fn main() {
    input! {
        r: i32,
    }

    if r <= 99 {
        println!("{}", 100 - r);
    } else if r <= 199 {
        println!("{}", 200 - r);
    } else if r <= 299 {
        println!("{}", 300 - r);
    }
}
