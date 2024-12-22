use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut santa_pos: (usize, usize),
        board: [Chars; h],
        t: Chars,
    }

    santa_pos = (santa_pos.0 - 1, santa_pos.1 - 1);

    let mut house = HashSet::new();

    for i in 0..t.len() {
        let (mut x, mut y) = santa_pos;
        match t[i] {
            'U' => {
                if x > 0 && board[x - 1][y] != '#' {
                    x -= 1;
                }
            }
            'D' => {
                if x < h - 1 && board[x + 1][y] != '#' {
                    x += 1;
                }
            }
            'L' => {
                if y > 0 && board[x][y - 1] != '#' {
                    y -= 1;
                }
            }
            'R' => {
                if y < w - 1 && board[x][y + 1] != '#' {
                    y += 1;
                }
            }
            _ => unreachable!(),
        }
        santa_pos = (x, y);
        if board[x][y] == '@' {
            house.insert((x, y));
        }
    }

    println!("{} {} {}", santa_pos.0 + 1, santa_pos.1 + 1, house.len());
}
