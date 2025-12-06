use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut right = a[0];
    let mut i = 0;

    while i < right && i < n {
        let reach = i + a[i];
        if reach > right {
            right = reach.min(n);
        }
        i += 1;
    }

    println!("{}", right);
}
