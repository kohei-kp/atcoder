use proconio::input;

fn main() {
    input!(
        b: i32,
        g: i32,
    );

    if b < g {
        println!("Glove");
    } else {
        println!("Bat");
    }
}
