use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    for i in 0..n {
        print!("{}", if (i + 1) % 3 == 0 { "x" } else { "o" });
    }
}
