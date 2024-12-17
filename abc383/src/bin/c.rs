use std::collections::VecDeque;

use proconio::{input, marker::Chars};

const DI: [i32; 4] = [-1, 0, 1, 0];
const DJ: [i32; 4] = [0, -1, 0, 1];

fn main() {
    input! {
        h: usize,
        w: usize,
        d: i32,
        board: [Chars; h],
    }

    let inf = 1_000_000_001;
    let mut dist = vec![vec![inf; w]; h];
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();

    for i in 0..h {
        for j in 0..w {
            if board[i][j] == 'H' {
                dist[i][j] = 0;
                queue.push_back((i as i32, j as i32));
            }
        }
    }

    while !queue.is_empty() {
        let (i, j) = queue.pop_front().unwrap();
        let d = dist[i as usize][j as usize];
        for k in 0..4 {
            let ni = i + DI[k];
            let nj = j + DJ[k];
            if ni < 0 || nj < 0 || ni >= h as i32 || nj >= w as i32 {
                continue;
            }
            if board[ni as usize][nj as usize] == '#' {
                continue;
            }
            if dist[ni as usize][nj as usize] != inf {
                continue;
            }
            dist[ni as usize][nj as usize] = d + 1;
            queue.push_back((ni, nj));
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if dist[i][j] <= d {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
