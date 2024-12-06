use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut _a = a.clone();
    _a.sort_by(|a, b| b.cmp(a));

    println!("{}", a.iter().position(|x| x == &_a[1]).unwrap() + 1);
}
