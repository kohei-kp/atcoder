use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars
    }

    let mut min = i32::MAX;

    for i in 0..=(n - m) {
        let mut c = 0;

        for j in 0..m {
            let s_d = s[i + j].to_digit(10).unwrap() as i32;
            let t_d = t[j].to_digit(10).unwrap() as i32;

            let diff = (s_d - t_d + 10) % 10;
            c += diff;
        }

        min = std::cmp::min(min, c);
    }

    println!("{}", min);
}
