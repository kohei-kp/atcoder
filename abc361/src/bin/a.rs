use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: i32,
        a: [i32; n],
    }

    [&a[..k], &[x], &a[k..]]
        .concat()
        .iter()
        .for_each(|x| print!("{} ", x));
}
