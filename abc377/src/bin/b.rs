use proconio::input;

fn main() {
    input! {
        board: [String; 8],
    }

    let mut result_board = board
        .clone()
        .iter()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut count = 0;

    for i in 0..8 {
        let row = board[i].chars().collect::<Vec<char>>();
        for j in 0..8 {
            if row[j] == '#' {
                for k in 0..8 {
                    result_board[i][k] = 'x';
                    result_board[k][j] = 'x';
                }
            }
        }
    }

    for row in result_board.iter() {
        count += row.iter().filter(|&&ch| ch == '.').count();
    }

    println!("{}", count);
}
