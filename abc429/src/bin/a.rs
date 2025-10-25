use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize
    }

    for i in 0..n {
        if i + 1 <= m {
            println!("OK")
        } else {
            println!("Too Many Requests");
        }
    }
}
