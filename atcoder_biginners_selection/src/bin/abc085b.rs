use proconio::input;

fn main() {
    input! {
        n: i32,
        mut d: [i32; n],
    }

    d.sort_by(|a, b| b.cmp(a));
    d.dedup();

    println!("{:?}", d.len());
}
