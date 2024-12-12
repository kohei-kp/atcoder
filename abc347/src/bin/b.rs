use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut ans: Vec<String> = vec![];

    for i in 0..s.len() {
        for j in i + 1..s.len() + 1 {
            let _s = &s[i..j];
            ans.push(_s.iter().collect());
        }
    }

    ans.sort();
    ans.dedup();
    println!("{}", ans.len());
}
