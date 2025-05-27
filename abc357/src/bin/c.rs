use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let mut grid = create_grid_level(n);
    if n == 0 {
        println!("#");
        return;
    }
    for level in (0..=n - 1).rev() {
        let min_grid = create_grid_level(level);
        for ii in 0..grid.len() {
            let i_mod = 3usize.pow(level);
            let index_i = if level == 0 { 0 } else { ii % i_mod };
            for jj in 0..grid[ii].len() {
                let index_j = if level == 0 { 0 } else { jj % i_mod };
                let s = min_grid[index_i][index_j];
                if grid[ii][jj] == '.' {
                    continue;
                } else {
                    grid[ii][jj] = s;
                }
            }
        }
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn create_grid_level(n: u32) -> Vec<Vec<char>> {
    let max_col = 3usize.pow(n);
    let max_row = max_col;
    let center_min = max_col / 3;
    let center_max = 2 * center_min;

    let mut grid = vec![vec!['.'; max_col]; max_row];

    for i in 0..max_row {
        for j in 0..max_col {
            if i < center_min || i >= center_max || j < center_min || j >= center_max {
                grid[i][j] = '#';
            } else {
                grid[i][j] = '.';
            }
        }
    }

    grid
}
