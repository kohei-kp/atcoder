use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        d: i64
    }

    let mut ps: Vec<(i64, i64)> = vec![];
    for _ in 0..n {
        input! {
            p: (i64, i64)
        }
        ps.push(p);
    }

    // BFS
    let mut queue = VecDeque::new();
    let mut ans = vec![false; n];
    ans[0] = true;
    queue.push_back(0);

    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();

        for u in 0..n {
            if near(v, u, &ps, d) {
                if ans[u] {
                    continue;
                }
                ans[u] = true;
                queue.push_back(u);
            }
        }
    }

    for i in 0..ans.len() {
        if ans[i] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn near(a: usize, b: usize, ps: &Vec<(i64, i64)>, d: i64) -> bool {
    let dx = ps[a].0 - ps[b].0;
    let dy = ps[a].1 - ps[b].1;
    dx * dx + dy * dy <= d * d
}
