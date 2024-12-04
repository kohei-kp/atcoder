use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i32; n],
    }

    let vec = [&a[n - k..], &a[..n - k]].concat();
    for i in 0..n {
        print!("{} ", vec[i]);
    }
}
