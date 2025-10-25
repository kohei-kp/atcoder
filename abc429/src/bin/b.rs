use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n]
    }

    for i in 0..n {
        let sum: usize = a
            .iter()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .map(|(_, &val)| val)
            .sum();

        if sum == m {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
