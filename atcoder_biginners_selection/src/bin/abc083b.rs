use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
    }

    let mut result = 0;

    for i in 1..=n {
        let sum = i
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .sum::<u32>() as i32;

        if a <= sum && sum <= b {
            result += i;
        }
    }

    println!("{}", result);
}
