use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i32,
        a: [i32;n]
    }

    println!("{}", a.iter().filter(|&&x| x >= l).count());
}
