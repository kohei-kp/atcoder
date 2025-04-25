use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut queue = VecDeque::new();

    for _ in 0..q {
        input! {
            q_type: usize,
        }
        match q_type {
            1 => {
                input! {
                    x: usize,
                }
                queue.push_back(x);
            }
            2 => {
                let first = queue.pop_front().unwrap();
                println!("{}", first);
            }
            _ => unreachable!(),
        }
    }
}
