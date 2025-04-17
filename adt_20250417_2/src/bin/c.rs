use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut ans = 0;

    for i in 0..s.len() {
        for j in i..s.len() {
            let mut t = s[i..=j].to_vec();
            let mut _t = t.clone();
            _t.reverse();

            if t == _t {
                ans = ans.max(t.len());
            }
        }
    }

    println!("{}", ans);
}
