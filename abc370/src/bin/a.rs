use proconio::input;

fn main() {
    input! {
        l: i32,
        r: i32,
    }

    if (l == 1 && r == 1) || (l == 0 && r == 0) {
        println!("Invalid");
    } else if l == 1 {
        println!("Yes");
    } else if r == 1 {
        println!("No");
    }
}
