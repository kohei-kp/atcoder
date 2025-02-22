use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut ss: [Chars ;n]
    }

    ss.sort_by(|a, b| a.len().cmp(&b.len()));

    for s in ss.iter() {
        print!("{}", s.iter().collect::<String>());
    }
}
