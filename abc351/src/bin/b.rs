use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [String; n],
        b: [String; n]
    }

    let board_a = a
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let board_b = b
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for i in 0..n {
        for j in 0..n {
            if board_a[i][j] != board_b[i][j] {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
}
