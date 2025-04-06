use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

const DI: [isize; 4] = [-1, 0, 1, 0];
const DJ: [isize; 4] = [0, -1, 0, 1];

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
        si: Usize1,
        sj: Usize1,
        ti: Usize1,
        tj: Usize1,
    }

    let inf = 1001001001;
    let mut dist = vec![vec![inf; w]; h];
    let mut used = vec![vec![false; w]; h];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

    push(si, sj, 0, 0, &mut q, &mut dist);

    while !q.is_empty() {
        let (i, j) = q.pop_front().unwrap();
        if used[i][j] {
            continue;
        }
        used[i][j] = true;
        let d = dist[i][j];

        for v in 0..4 {
            let ni = i as isize + DI[v];
            let nj = j as isize + DJ[v];
            if ni < 0 || nj < 0 || ni >= h as isize || nj >= w as isize {
                continue;
            }
            if s[ni as usize][nj as usize] == '.' {
                push(ni as usize, nj as usize, d, 0, &mut q, &mut dist);
            }
        }
        for v in 0..4 {
            let mut ni = i as isize;
            let mut nj = j as isize;
            for _ in 0..2 {
                ni += DI[v];
                nj += DJ[v];
                if ni < 0 || nj < 0 || ni >= h as isize || nj >= w as isize {
                    break;
                }
                push(ni as usize, nj as usize, d + 1, 1, &mut q, &mut dist);
            }
        }
    }

    let ans = dist[ti][tj];
    println!("{}", ans);
}

fn push(
    i: usize,
    j: usize,
    d: usize,
    cost: usize,
    q: &mut VecDeque<(usize, usize)>,
    dist: &mut [Vec<usize>],
) {
    if dist[i][j] <= d {
        return;
    }

    dist[i][j] = d;

    if cost == 0 {
        q.push_front((i, j));
    } else {
        q.push_back((i, j));
    }
}
