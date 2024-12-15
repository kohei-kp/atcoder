use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    for i in 0..n {
        for j in 0..n {
            input! {
                v: i32,
            }
            if v == 1 {
                print!("{} ", j + 1);
            }
        }
        println!();
    }
}
