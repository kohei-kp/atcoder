use proconio::input;

fn main() {
    input! {
        x: f32,
    }

    if x >= 38.0 {
        println!("1");
    } else if (37.5..38.0).contains(&x) {
        println!("2");
    } else {
        println!("3");
    }
}
