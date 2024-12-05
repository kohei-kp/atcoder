use proconio::input;

fn main() {
    input! {
        n: i32,
        t: i32,
        a: i32,
    }

    let remain = n - (t + a);

    if t > a + remain || a > t + remain {
        println!("Yes");
    } else {
        println!("No");
    }
}
