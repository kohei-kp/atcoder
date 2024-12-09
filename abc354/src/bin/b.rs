use proconio::input;

fn main() {
    input! {
        n: usize,
        mut pairs: [(String, i32); n]
    }

    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    let sum = pairs.iter().map(|x| x.1).sum::<i32>();
    let ans = sum % n as i32;
    println!("{}", pairs[ans as usize].0);
}
