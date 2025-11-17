use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize, // 高橋くんが所持する飴
        d: usize, // 高橋くんが所持するクッキー
    }

    if c >= a && d >= b || c < a {
        println!("No");
    } else {
        println!("Yes");
    }
}
