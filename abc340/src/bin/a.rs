use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        d: i32,
    }

    let mut current = a;
    while current <= b {
        print!("{} ", current);
        current += d;
    }
}
