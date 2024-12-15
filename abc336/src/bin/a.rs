use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    let mut loong = "L".to_string();

    for _ in 0..n {
        loong += "o";
    }
    loong += "ng";

    println!("{}", loong);
}
