use proconio::input;

fn main() {
    input! {
        d: i32,
        f: i32,
    }

    let ans = (f + 7 - (d % 7)) % 7;
    if ans == 0 {
        println!("7");
    } else {
        println!("{}", ans);
    }
}
