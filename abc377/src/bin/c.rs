use std::collections::{HashSet, VecDeque};

use proconio::input;

const DI: [i64; 8] = [-2, -1, 1, 2, 2, 1, -1, -2];
const DJ: [i64; 8] = [1, 2, 2, 1, -1, -2, -2, -1];

fn main() {
    input! {
        n: usize,
        m: usize,
        pos: [(usize, usize); m],
    }

    let mut queue = VecDeque::new();
    let mut map = HashSet::new();

    for (x, y) in pos {
        queue.push_back((x - 1, y - 1));
        map.insert((x - 1, y - 1));
    }

    let mut ans = (n * n) as i64 - m as i64;

    for _ in 0..m {
        let p = queue.pop_front().unwrap();

        for v in 0..8 {
            let ni = p.0 as i64 + DI[v];
            let nj = p.1 as i64 + DJ[v];

            if ni < 0 || nj < 0 || ni >= n as i64 || nj >= n as i64 {
                continue;
            }

            if map.contains(&(ni as usize, nj as usize)) {
                continue;
            }
            map.insert((ni as usize, nj as usize));

            ans -= 1;
        }
    }

    println!("{}", ans);
}
