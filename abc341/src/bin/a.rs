use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    for i in 0..(n + (n + 1)) {
        if i % 2 == 0 {
            print!("{}", 1);
        } else {
            print!("{}", 0);
        }
    }
}
