use proconio::input;

fn main() {
    input! {
        a: f32,
        b: f32,
    }
    let result = (a / b).round() as i64;
    println!("{}", result);
}
