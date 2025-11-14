use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    if a + b == c || a + c == b || b + c == a || (a == b && b == c && a == c) {
        println!("Yes");
    } else {
        println!("No");
    }
}
