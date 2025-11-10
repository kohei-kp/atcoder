use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = String::new();

    for i in 0..=n {
        ans.push('-');
        for j in 1..=9 {
            if n % j == 0 && i % (n / j) == 0 {
                ans.pop();
                ans.push(char::from_digit(j as u32, 10).unwrap());
                break;
            }
        }
    }

    println!("{}", ans);
}
