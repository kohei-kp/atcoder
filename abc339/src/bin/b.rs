use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    }

    let mut board = vec![vec!["."; w]; h];
    let mut pos = (0, 0);
    let mut dir = 0;

    for i in 0..n {
        if board[pos.0][pos.1] == "." {
            board[pos.0][pos.1] = "#";
            dir = (dir + 1) % 4;
        } else {
            board[pos.0][pos.1] = ".";
            dir = (dir + 3) % 4;
        }
        if dir == 0 {
            pos = (if pos.0 == 0 { h - 1 } else { pos.0 - 1 }, pos.1);
        } else if dir == 1 {
            pos = (pos.0, if pos.1 == w - 1 { 0 } else { pos.1 + 1 });
        } else if dir == 2 {
            pos = (if pos.0 == h - 1 { 0 } else { pos.0 + 1 }, pos.1);
        } else if dir == 3 {
            pos = (pos.0, if pos.1 == 0 { w - 1 } else { pos.1 - 1 });
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", board[i][j]);
        }
        println!();
    }
}
