use proconio::input;

fn main() {
    input! {
        mut a: [i32; 5]
    }

    let mut count = 0;
    for i in 0..4 {
        let now = a[i];
        let next = a[i + 1];
        if now > next {
            count += 1;
            a[i] = next;
            if i != 4 {
                a[i + 1] = now;
            }
        }
    }
    println!("{}", if count == 1 { "Yes" } else { "No" });
}
