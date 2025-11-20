use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut queue = VecDeque::new();
    for i in 0..n {
        queue.push_back(s[i]);

        if queue.iter().collect::<String>() == "ABC".to_string() {
            println!("{}", i - 1);
            return;
        }

        if queue.len() == 3 {
            queue.pop_front();
        }
    }

    println!("-1");
}
