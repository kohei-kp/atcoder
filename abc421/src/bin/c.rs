use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    // 交互になるパターンは2つ
    // ABAB or BABA
    let a_positions: Vec<usize> = s
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if c == 'A' { Some(i) } else { None })
        .collect();
    let b_positions: Vec<usize> = s
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if c == 'B' { Some(i) } else { None })
        .collect();

    let ab_moves = a_positions
        .iter()
        .enumerate()
        .map(|(i, &pos)| (2 * i) as isize - pos as isize)
        .map(|diff| diff.abs())
        .sum::<isize>();
    let ba_moves = b_positions
        .iter()
        .enumerate()
        .map(|(i, &pos)| (2 * i) as isize - pos as isize)
        .map(|diff| diff.abs())
        .sum::<isize>();
    println!("{}", ab_moves.min(ba_moves));
}
