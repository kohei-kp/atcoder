use proconio::input;

fn main() {
    input! {
        y: i32,
    }

    if y % 4 != 0 {
        println!("365");
    } else if y % 4 == 0 && y % 100 != 0 {
        println!("366");
    } else if y % 100 == 0 && y % 400 != 0 {
        println!("365");
    } else if y % 400 == 0 {
        println!("366");
    }
}
