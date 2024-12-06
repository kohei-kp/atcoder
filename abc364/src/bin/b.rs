use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut s_i: usize,
        mut s_j: usize,
    }

    // -1して配列のindexに合わせる
    s_i -= 1;
    s_j -= 1;

    let mut board = vec![vec!["".to_string(); w]; h];
    for i in 0..h {
        input! {
            row: String,
        }
        let _row = row.chars().collect::<Vec<char>>();

        for j in 0..w {
            board[i][j] = _row[j].to_string();
        }
    }

    input! {
        x: String
    }

    for x in x.chars().collect::<Vec<char>>() {
        match x {
            'L' => {
                if s_j > 0 && board[s_i][s_j - 1] != "#" {
                    s_j -= 1;
                }
            }
            'R' => {
                if s_j < w - 1 && board[s_i][s_j + 1] != "#" {
                    s_j += 1;
                }
            }
            'U' => {
                if s_i > 0 && board[s_i - 1][s_j] != "#" {
                    s_i -= 1;
                }
            }
            'D' => {
                if s_i < h - 1 && board[s_i + 1][s_j] != "#" {
                    s_i += 1;
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{} {}", s_i + 1, s_j + 1);
}
