use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
    }

    let mut a = (1..(n + 1) as i32).collect::<Vec<i32>>();
    a[l - 1..r].reverse();

    a.iter().for_each(|x| print!("{} ", x));
}
