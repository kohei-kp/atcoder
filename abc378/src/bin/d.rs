use proconio::{input, marker::Chars};

const DI: [i32; 4] = [-1, 0, 1, 0];
const DJ: [i32; 4] = [0, -1, 0, 1];

fn main() {
    input! {
        h: usize,
        w: usize,
        k: i32,
        mut board: [Chars; h],
    }

    let mut ans = 0;
    for bi in 0..h {
        for bj in 00..w {
            if board[bi][bj] == '#' {
                continue;
            }
            ans += dfs(bi, bj, h, w, k, &mut board);
        }
    }

    println!("{}", ans);
}

fn dfs(i: usize, j: usize, h: usize, w: usize, k: i32, board: &mut Vec<Vec<char>>) -> i32 {
    if k == 0 {
        return 1;
    }

    let v = board[i][j];
    board[i][j] = '*';

    let mut res = 0;
    for v in 0..4 {
        let ni = i as i32 + DI[v];
        let nj = j as i32 + DJ[v];
        if ni < 0 || nj < 0 || ni >= h as i32 || nj >= w as i32 {
            continue;
        }
        if board[ni as usize][nj as usize] != '.' {
            continue;
        }
        res += dfs(ni as usize, nj as usize, h, w, k - 1, board);
    }

    board[i][j] = v;

    res
}
