use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }

    let mut x: Vec<Vec<char>> = vec![];

    let n = s.len();

    while String::from_iter(s.clone()) != String::from_iter(t.clone()) {
        let mut best = vec![];
        for i in 0..n {
            if s[i] == t[i] {
                continue;
            }
            let mut ns = s.clone();
            ns[i] = t[i];

            if best.is_empty() {
                best = ns
            } else {
                best = best.min(ns);
            }
        }
        s = best.clone();
        x.push(best.clone());
    }

    println!("{}", x.len());
    for i in x {
        println!("{}", String::from_iter(i));
    }
}
