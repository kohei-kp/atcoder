use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
        mut b: [i32; n],
    }

    a.sort_by(|a, b| b.cmp(a));
    b.sort_by(|a, b| b.cmp(a));
    println!("{}", a[0] + b[0]);
}
