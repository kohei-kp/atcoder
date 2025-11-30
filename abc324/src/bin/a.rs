use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32;n]
    }

    a.sort();
    a.dedup();

    println!("{}", if a.len() == 1 { "Yes" } else { "No" });
}
