use proconio::input;

fn main() {
    input!(
        n: i32,
        scores: [[i32; 2]; n],
    );

    let mut total_a = 0;
    let mut total_b = 0;
    for score in &scores {
        total_a += score[0];
        total_b += score[1];
    }

    if total_a > total_b {
        println!("Takahashi");
    } else if total_a < total_b {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}
