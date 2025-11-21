use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    let mut ranking = vec![];

    for i in 0..n {
        let player_no = i + 1;
        let win_count = s[i].chars().filter(|c| c == &'o').count();
        ranking.push((player_no, win_count));
    }

    ranking.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    for i in 0..n {
        print!("{} ", ranking[i].0);
    }
    println!();
}
