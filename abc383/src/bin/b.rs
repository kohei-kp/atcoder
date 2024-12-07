use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
    }

    let mut board = vec![vec!["".to_string(); w]; h];

    for i in 0..h {
        input! {
            row: String,
        }
        for (j, c) in row.chars().enumerate() {
            board[i][j] = c.to_string();
        }
    }

    let mut cell_pos: Vec<(usize, usize)> = vec![];

    for i in 0..h {
        for j in 0..w {
            let cell = &board[i][j];
            if cell == "#" {
                continue;
            }
            cell_pos.push((i, j));
        }
    }

    let mut counts: Vec<i32> = vec![];
    for i in 0..cell_pos.len() {
        let p1 = cell_pos[i];
        let mut ps: Vec<String> = vec![];

        let mut count = 0;
        let cells = search(p1, d, &board);
        for cell in cells {
            let (x, y) = cell;
            if board[x][y] == "." {
                count += 1;
                ps.push(format!("{},{}", x, y));
            }
        }

        let mut in_counts: Vec<i32> = vec![];
        for j in 0..cell_pos.len() {
            let mut in_count = 0;
            if i == j {
                continue;
            }
            let p2 = cell_pos[j];
            let cells = search(p2, d, &board);
            for cell in cells {
                let (x, y) = cell;
                if board[x][y] == "." {
                    if ps.contains(&format!("{},{}", x, y)) {
                        continue;
                    }
                    in_count += 1;
                }
            }
            in_counts.push(in_count);
        }
        counts.push(count + in_counts.iter().max().unwrap());
    }
    println!("{}", counts.iter().max().unwrap());
}

fn search(p: (usize, usize), d: usize, board: &Vec<Vec<String>>) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let (x, y) = p;
    let h = board.len();
    let w = board[0].len();

    for i in 0..h {
        for j in 0..w {
            if (x as i32 - i as i32).abs() + (y as i32 - j as i32).abs() <= d as i32 {
                result.push((i, j));
            }
        }
    }

    result
}
