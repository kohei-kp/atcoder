use proconio::{input, marker::Chars};
use std::cmp::max;

fn main() {
    input! {
        n: usize,
    }

    let mut h = vec![];
    let mut max_len = 0;
    for _i in 0..n {
        input! {
            s: Chars
        }
        let s_len = s.clone().len();
        h.push(s);
        max_len = max(max_len, s_len);
    }

    let mut v: Vec<Vec<String>> = vec![vec![String::from("*"); n]; max_len];

    for i in 0..n {
        let row = h[i].clone();
        for j in 0..row.len() {
            v[j][n - 1 - i] = row[j].to_string();
        }
    }

    for i in 0..v.len() {
        let row = v[i].clone();
        let mut str = row.join("");

        loop {
            let c = str.chars().last().unwrap();
            if c != '*' {
                break;
            }
            str.pop();
        }
        println!("{}", str);
    }
}
