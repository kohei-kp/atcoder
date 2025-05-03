use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut grid_s: [Chars; n],
        grid_t: [Chars; n],
    }

    let mut ans = 1000_000_000;
    for i in 0..4 {
        // grid_sとgrid_tの差分を求める
        let mut diff_count = 0;
        for j in 0..n {
            for k in 0..n {
                if grid_s[j][k] != grid_t[j][k] {
                    diff_count += 1;
                }
            }
        }
        ans = ans.min(diff_count + i);
        // grid_sを90度回転させる
        grid_s = rotate_90(&grid_s);
    }

    println!("{}", ans);
}

fn rotate_90(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let n = grid.len();
    let mut rotated = vec![vec![' '; n]; n];

    for i in 0..n {
        for j in 0..n {
            rotated[j][n - 1 - i] = grid[i][j];
        }
    }

    rotated
}
