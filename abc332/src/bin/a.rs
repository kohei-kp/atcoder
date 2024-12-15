use proconio::input;

fn main() {
    input! {
        n: usize,
        s: i32,
        k: i32,
        pq: [(i32, i32); n]
    }

    let mut sum = 0;

    for i in 0..n {
        sum += pq[i].0 * pq[i].1;
    }

    println!("{}", if sum >= s { sum } else { sum + k });
}
