use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        d: i64,
        ps: [(i64, i64); n],
    }

    let near = |a: usize, b: usize| -> bool {
        let (ax, ay) = ps[a];
        let (bx, by) = ps[b];
        let dx = ax - bx;
        let dy = ay - by;
        dx * dx + dy * dy <= d * d
    };

    // BFS
    let mut queue = VecDeque::new();
    let mut ans = vec![false; n];
    ans[0] = true;
    queue.push_back(0);

    while let Some(v) = queue.pop_front() {
        for u in 0..n {
            if near(v, u) {
                if ans[u] {
                    continue;
                }
                ans[u] = true;
                queue.push_back(u);
            }
        }
    }

    for ok in ans {
        println!("{}", if ok { "Yes" } else { "No" })
    }
}
