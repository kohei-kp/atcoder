use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n]
    }

    a.sort();
    a.dedup();
    println!("{}", a.len());
    for i in 0..a.len() {
        print!("{} ", a[i]);
    }
}
