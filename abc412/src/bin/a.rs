use proconio::input;

fn main() {
    input! {
        n: usize,
        tasks: [(usize, usize); n],
    }

    let mut count = 0;

    for i in 0..n {
        let (a, b) = tasks[i];
        if b > a {
            count += 1;
        }
    }

    println!("{}", count);
}
