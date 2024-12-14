use proconio::input;

fn main() {
    let mut numbers: Vec<i32> = vec![];
    for _ in 0..100 {
        input! {
            n: i32,
        }
        numbers.push(n);
        if n == 0 {
            break;
        }
    }

    numbers.reverse();
    numbers.iter().for_each(|x| println!("{} ", x));
}
