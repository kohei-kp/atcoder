use proconio::input;

fn main() {
    input! {
        r: usize,
        x: usize
    }

    if x == 1 {
        if r >= 1600 && r <= 2999 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else if x == 2 {
        if r >= 1200 && r <= 2399 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
