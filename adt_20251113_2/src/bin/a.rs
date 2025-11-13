use proconio::input;

fn main() {
    input! {
        a: [usize; 9],
        b: [usize; 8]
    }

    let sum_a: usize = a.iter().sum();
    let sum_b: usize = b.iter().sum();

    println!("{}", (sum_a - sum_b) + 1);
}
