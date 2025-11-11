use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }

    println!("{}", a.pow(b as u32) + b.pow(a as u32));
}
