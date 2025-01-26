use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
    }

    let mut board = vec![];
    for _ in 0..h {
        input! {
            row: Chars
        }
        board.push(row);
    }

    let mut li = h;
    let mut ri = 0;
    let mut lj = w;
    let mut rj = 0;

    for i in 0..h {
        for j in 0..w {
            if board[i][j] == '#' {
                li = std::cmp::min(li, i);
                ri = std::cmp::max(ri, i);
                lj = std::cmp::min(lj, j);
                rj = std::cmp::max(rj, j);
            }
        }
    }

    for i in li..=ri {
        for j in lj..=rj {
            if board[i][j] == '.' {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
