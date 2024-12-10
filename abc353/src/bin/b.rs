use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        a: [i32; n]
    }

    let mut count = 0;

    let mut _k = k;

    for i in 0..n {
        let current_group = a[i];
        let next_group = if i == n - 1 { 0 } else { a[i + 1] };

        _k -= current_group;
        if _k < next_group || next_group == 0 {
            count += 1;
            _k = k;
        } else {
            continue;
        }
    }
    println!("{}", count);
}
