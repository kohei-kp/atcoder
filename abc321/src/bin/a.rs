use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars
    }

    let mut prev = n[0];
    let mut flg = true;
    for i in 1..n.len() {
        if prev > n[i] {
            flg = true;
        } else {
            flg = false;
            break;
        }
        prev = n[i];
    }

    if flg {
        println!("Yes");
    } else {
        println!("No");
    }
}
