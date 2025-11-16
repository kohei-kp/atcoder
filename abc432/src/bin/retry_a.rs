use proconio::input;

fn main() {
    input! {
        mut s: [i64; 3]
    }

    s.sort_by(|a, b| b.cmp(a));
    for c in s {
        print!("{}", c.to_string());
    }
    println!();
}
