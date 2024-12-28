use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut count_zero = 0;
    let mut other_count = 0;
    let mut zeros = vec![];
    for i in 0..s.len() {
        if s[i] == '0' {
            count_zero += 1;
        } else {
            zeros.push(count_zero);
            count_zero = 0;
            other_count += 1;
        }
    }
    zeros.push(count_zero);

    let mut ans = 0;
    for i in 0..zeros.len() {
        ans += zeros[i] % 2 + zeros[i] / 2;
    }
    ans += other_count;

    println!("{}", ans);
}
