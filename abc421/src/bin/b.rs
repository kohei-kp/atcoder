use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize
    }

    let mut a = vec![0; 11];
    a[1] = x;
    a[2] = y;
    for i in 3..=10 {
        a[i] = rev((a[i - 1] + a[i - 2]).to_string()).parse().unwrap();
    }
    println!("{}", a[10]);
}

fn rev(s: String) -> String {
    s.chars().rev().collect()
}
