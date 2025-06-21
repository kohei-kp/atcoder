use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [i32; n - 1],
    }

    for i in 0..n - 1 {
        let mut sum = 0;
        for j in i..n - 1 {
            sum += d[j];
            print!("{} ", sum);
        }
        println!();
    }
}
