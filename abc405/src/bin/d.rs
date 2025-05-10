use proconio::{input, marker::Chars};
const DI: [i32; 4] = [1, 0, -1, 0];
const DJ: [i32; 4] = [0, 1, 0, -1];

fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],
    }

    // goalのpositionを探す
    let mut goal = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == 'E' {
                goal = (i, j);
            }
        }
    }

    let mut ans = vec![vec!['#'; w]; h];
    let mut queue = vec![(goal.0, goal.1)];

    ans[goal.0][goal.1] = 'E';
    while !queue.is_empty() {
        let (i, j) = queue.remove(0);
        for d in 0..4 {
            let ni = i as i32 + DI[d];
            let nj = j as i32 + DJ[d];

            if ni < 0 || ni >= h as i32 || nj < 0 || nj >= w as i32 {
                continue;
            }

            let ni = ni as usize;
            let nj = nj as usize;

            if ans[ni][nj] == '#' && grid[ni][nj] != '#' {
                let move_dir = match d {
                    0 => '^',
                    1 => '<',
                    2 => 'v',
                    3 => '>',
                    _ => unreachable!(),
                };
                ans[ni][nj] = move_dir;
                queue.push((ni, nj));
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", ans[i][j]);
        }
        println!();
    }
}
