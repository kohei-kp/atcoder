use proconio::input;

fn main() {
    input! {
        k: u32,
        a: String,
        b: String,
    }

    let _a = u64::from_str_radix(&a, k).unwrap();
    let _b = u64::from_str_radix(&b, k).unwrap();

    println!("{}", _a * _b);
}
