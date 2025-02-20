use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut ans = 0;
    for i in 0..s.len() {
        for j in i..s.len() {
            for k in j..s.len() {
                if j - i == k - j && s[i] == 'A' && s[j] == 'B' && s[k] == 'C' {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
