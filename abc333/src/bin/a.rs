use proconio::input;

fn main() {
    input!(
        n: i32
    );

    let mut s = String::new();
    for _ in 0..n {
        s.push_str(&n.to_string());
    }

    println!("{}", s);
}
