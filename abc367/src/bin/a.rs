use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32
    }

    if b < c {
        if 0 <= a && a < b || c < a && a <= 23 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        if c < a && a < b {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
