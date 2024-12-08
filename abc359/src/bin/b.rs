use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n * 2],
    }

    let mut prev = a[0];
    let mut count = 0;
    for i in 1..(n * 2) - 1 {
        let current = a[i];
        let next = a[i + 1];
        if prev == next && prev != current {
            count += 1;
        }
        prev = current;
    }
    println!("{}", count);
}
