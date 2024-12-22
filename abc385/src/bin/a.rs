use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }

    if a == b && b == c || a + b == c || a + c == b || b + c == a {
        println!("Yes");
    } else {
        println!("No");
    }
}
