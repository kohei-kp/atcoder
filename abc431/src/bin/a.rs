use proconio::input;

fn main() {
    input! {
        h: i32,
        b: i32,
    }

    if h < b {
        println!("0");
        return;
    }

    println!("{}", (b - h).abs());
}
