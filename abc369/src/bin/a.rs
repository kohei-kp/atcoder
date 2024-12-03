use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    let diff = (a - b).abs();

    if diff == 0 {
        println!("{}", 1);
    } else if diff % 2 == 0 {
        println!("{}", 3);
    } else {
        println!("{}", 2);
    }
}
