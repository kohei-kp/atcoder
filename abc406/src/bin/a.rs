use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    if c < a {
        println!("Yes");
    } else if c == a {
        if d <= b {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}
