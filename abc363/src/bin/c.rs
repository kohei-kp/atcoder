use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: Chars
    }

    let unique_chars = s.iter().unique().count();
    if unique_chars == n {
        let mut ans = 1;
        for i in 1..=n {
            ans *= i;
        }
        println!("{}", ans);
        return;
    }

    let mut ans = 0;
    s.sort();
    for p in s.iter().permutations(n).unique() {
        let mut ok = true;

        for i in 0..n - k + 1 {
            let t = p[i..i + k].iter().cloned().collect::<String>();
            let rt = t.chars().rev().collect::<String>();
            if t == rt {
                ok = false;
            }
        }
        if ok {
            ans += 1;
        }
    }

    println!("{}", ans);
}
