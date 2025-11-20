use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: String, // prefix or suffix
        t: String, // base
    }

    if t[0..n] == s && t[m - n..m] == s {
        println!("0");
    } else if t[0..n] == s {
        println!("1");
    } else if t[m - n..m] == s {
        println!("2");
    } else {
        println!("3");
    }
}
