use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut ans = 0;

    for i in 0..n {
        let target = a[i] / 2;
        let r = a
            .binary_search_by(|v| {
                if v <= &target {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            })
            .unwrap_or_else(|pos| pos);

        ans += r;
    }

    println!("{}", ans);
}
