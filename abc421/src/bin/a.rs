use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut s = vec![];
    for _ in 0..n {
        input! {
            a: String,
        }
        s.push(a);
    }

    input! {
        x: usize,
        y: String
    }

    if s[x - 1] == y.to_string() {
        println!("Yes");
    } else {
        println!("No");
    }
}
