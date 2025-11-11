use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        mut a: [usize; n - 1]
    }

    for score in 0..=100 {
        let mut copy = a.clone();
        copy.push(score);
        copy.sort();
        let result: usize = copy[1..copy.len() - 1].iter().sum();
        if result >= x {
            println!("{}", score);
            return;
        }
    }
    println!("-1");
}
