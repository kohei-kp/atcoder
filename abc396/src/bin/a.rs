use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut prev = a[0];
    let mut count = 1;
    for i in 1..n {
        if a[i] == prev {
            count += 1;
            if count >= 3 {
                println!("Yes");
                return;
            }
        } else {
            count = 1;
        }
        prev = a[i];
    }

    println!("No");
}
