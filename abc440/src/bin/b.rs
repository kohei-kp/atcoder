use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [i32; n]
    }

    let mut ranks = vec![];
    for i in 0..n {
        ranks.push((i + 1, t[i]));
    }

    ranks.sort_by(|a, b| a.1.cmp(&b.1));

    for i in 0..3 {
        print!("{} ", ranks[i].0);
    }
    println!();
}
